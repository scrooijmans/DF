# 

opendal/services/swift/

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
26use super::SWIFT_SCHEME;
27use super::core::*;
28use super::delete::SwfitDeleter;
29use super::error::parse_error;
30use super::lister::SwiftLister;
31use super::writer::SwiftWriter;
32use crate::raw::*;
33use crate::services::SwiftConfig;
34use crate::*;
35
36/// [OpenStack Swift](https://docs.openstack.org/api-ref/object-store/#)'s REST API support.
37/// For more information about swift-compatible services, refer to [Compatible Services](#compatible-services).
38#[doc = include_str!("docs.md")]
39#[doc = include_str!("compatible_services.md")]
40#[derive(Default, Clone)]
41pub struct SwiftBuilder {
42    pub(super) config: SwiftConfig,
43}
44
45impl Debug for SwiftBuilder {
46    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
47        let mut d = f.debug_struct("SwiftBuilder");
48        d.field("config", &self.config);
49        d.finish_non_exhaustive()
50    }
51}
52
53impl SwiftBuilder {
54    /// Set the remote address of this backend
55    ///
56    /// Endpoints should be full uri, e.g.
57    ///
58    /// - `http://127.0.0.1:8080/v1/AUTH_test`
59    /// - `http://192.168.66.88:8080/swift/v1`
60    /// - `https://openstack-controller.example.com:8080/v1/ccount`
61    ///
62    /// If user inputs endpoint without scheme, we will
63    /// prepend `https://` to it.
64    pub fn endpoint(mut self, endpoint: &str) -> Self {
65        self.config.endpoint = if endpoint.is_empty() {
66            None
67        } else {
68            Some(endpoint.trim_end_matches('/').to_string())
69        };
70        self
71    }
72
73    /// Set container of this backend.
74    ///
75    /// All operations will happen under this container. It is required. e.g. `snapshots`
76    pub fn container(mut self, container: &str) -> Self {
77        self.config.container = if container.is_empty() {
78            None
79        } else {
80            Some(container.trim_end_matches('/').to_string())
81        };
82        self
83    }
84
85    /// Set root of this backend.
86    ///
87    /// All operations will happen under this root.
88    pub fn root(mut self, root: &str) -> Self {
89        self.config.root = if root.is_empty() {
90            None
91        } else {
92            Some(root.to_string())
93        };
94
95        self
96    }
97
98    /// Set the token of this backend.
99    ///
100    /// Default to empty string.
101    pub fn token(mut self, token: &str) -> Self {
102        if !token.is_empty() {
103            self.config.token = Some(token.to_string());
104        }
105        self
106    }
107}
108
109impl Builder for SwiftBuilder {
110    type Config = SwiftConfig;
111
112    /// Build a SwiftBackend.
113    fn build(self) -> Result<impl Access> {
114        debug!("backend build started: {:?}", &self);
115
116        let root = normalize_root(&self.config.root.unwrap_or_default());
117        debug!("backend use root {root}");
118
119        let endpoint = match self.config.endpoint {
120            Some(endpoint) => {
121                if endpoint.starts_with("http") {
122                    endpoint
123                } else {
124                    format!("https://{endpoint}")
125                }
126            }
127            None => {
128                return Err(Error::new(
129                    ErrorKind::ConfigInvalid,
130                    "missing endpoint for Swift",
131                ));
132            }
133        };
134        debug!("backend use endpoint: {}", &endpoint);
135
136        let container = match self.config.container {
137            Some(container) => container,
138            None => {
139                return Err(Error::new(
140                    ErrorKind::ConfigInvalid,
141                    "missing container for Swift",
142                ));
143            }
144        };
145
146        let token = self.config.token.unwrap_or_default();
147
148        Ok(SwiftBackend {
149            core: Arc::new(SwiftCore {
150                info: {
151                    let am = AccessorInfo::default();
152                    am.set_scheme(SWIFT_SCHEME)
153                        .set_root(&root)
154                        .set_native_capability(Capability {
155                            stat: true,
156                            read: true,
157
158                            write: true,
159                            write_can_empty: true,
160                            write_with_user_metadata: true,
161
162                            delete: true,
163
164                            list: true,
165                            list_with_recursive: true,
166
167                            shared: true,
168
169                            ..Default::default()
170                        });
171                    am.into()
172                },
173                root,
174                endpoint,
175                container,
176                token,
177            }),
178        })
179    }
180}
181
182/// Backend for Swift service
183#[derive(Debug, Clone)]
184pub struct SwiftBackend {
185    core: Arc<SwiftCore>,
186}
187
188impl Access for SwiftBackend {
189    type Reader = HttpBody;
190    type Writer = oio::OneShotWriter<SwiftWriter>;
191    type Lister = oio::PageLister<SwiftLister>;
192    type Deleter = oio::OneShotDeleter<SwfitDeleter>;
193
194    fn info(&self) -> Arc<AccessorInfo> {
195        self.core.info.clone()
196    }
197
198    async fn stat(&self, path: &str, _args: OpStat) -> Result<RpStat> {
199        let resp = self.core.swift_get_metadata(path).await?;
200
201        match resp.status() {
202            StatusCode::OK | StatusCode::NO_CONTENT => {
203                let headers = resp.headers();
204                let mut meta = parse_into_metadata(path, headers)?;
205                let user_meta = parse_prefixed_headers(headers, "x-object-meta-");
206                if !user_meta.is_empty() {
207                    meta = meta.with_user_metadata(user_meta);
208                }
209
210                Ok(RpStat::new(meta))
211            }
212            _ => Err(parse_error(resp)),
213        }
214    }
215
216    async fn read(&self, path: &str, args: OpRead) -> Result<(RpRead, Self::Reader)> {
217        let resp = self.core.swift_read(path, args.range(), &args).await?;
218
219        let status = resp.status();
220
221        match status {
222            StatusCode::OK | StatusCode::PARTIAL_CONTENT => Ok((RpRead::new(), resp.into_body())),
223            _ => {
224                let (part, mut body) = resp.into_parts();
225                let buf = body.to_buffer().await?;
226                Err(parse_error(Response::from_parts(part, buf)))
227            }
228        }
229    }
230
231    async fn write(&self, path: &str, args: OpWrite) -> Result<(RpWrite, Self::Writer)> {
232        let writer = SwiftWriter::new(self.core.clone(), args.clone(), path.to_string());
233
234        let w = oio::OneShotWriter::new(writer);
235
236        Ok((RpWrite::default(), w))
237    }
238
239    async fn delete(&self) -> Result<(RpDelete, Self::Deleter)> {
240        Ok((
241            RpDelete::default(),
242            oio::OneShotDeleter::new(SwfitDeleter::new(self.core.clone())),
243        ))
244    }
245
246    async fn list(&self, path: &str, args: OpList) -> Result<(RpList, Self::Lister)> {
247        let l = SwiftLister::new(
248            self.core.clone(),
249            path.to_string(),
250            args.recursive(),
251            args.limit(),
252        );
253
254        Ok((RpList::default(), oio::PageLister::new(l)))
255    }
256
257    async fn copy(&self, from: &str, to: &str, _args: OpCopy) -> Result<RpCopy> {
258        // cannot copy objects larger than 5 GB.
259        // Reference: https://docs.openstack.org/api-ref/object-store/#copy-object
260        let resp = self.core.swift_copy(from, to).await?;
261
262        let status = resp.status();
263
264        match status {
265            StatusCode::CREATED | StatusCode::OK => Ok(RpCopy::default()),
266            _ => Err(parse_error(resp)),
267        }
268    }
269}
```
