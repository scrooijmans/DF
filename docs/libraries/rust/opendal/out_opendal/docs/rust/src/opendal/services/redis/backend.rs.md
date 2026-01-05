# 

opendal/services/redis/

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
20use std::path::PathBuf;
21use std::time::Duration;
22
23use http::Uri;
24use redis::Client;
25use redis::ConnectionAddr;
26use redis::ConnectionInfo;
27use redis::ProtocolVersion;
28use redis::RedisConnectionInfo;
29use redis::cluster::ClusterClientBuilder;
30use tokio::sync::OnceCell;
31
32use super::REDIS_SCHEME;
33use super::core::*;
34use super::delete::RedisDeleter;
35use super::writer::RedisWriter;
36use crate::raw::oio;
37use crate::raw::*;
38use crate::services::RedisConfig;
39use crate::*;
40const DEFAULT_REDIS_ENDPOINT: &str = "tcp://127.0.0.1:6379";
41const DEFAULT_REDIS_PORT: u16 = 6379;
42
43/// [Redis](https://redis.io/) services support.
44#[doc = include_str!("docs.md")]
45#[derive(Clone, Default)]
46pub struct RedisBuilder {
47    pub(super) config: RedisConfig,
48}
49
50impl Debug for RedisBuilder {
51    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
52        let mut d = f.debug_struct("RedisBuilder");
53
54        d.field("config", &self.config);
55        d.finish_non_exhaustive()
56    }
57}
58
59impl RedisBuilder {
60    /// set the network address of redis service.
61    ///
62    /// currently supported schemes:
63    /// - no scheme: will be seen as "tcp"
64    /// - "tcp" or "redis": unsecured redis connections
65    /// - "rediss": secured redis connections
66    /// - "unix" or "redis+unix": unix socket connection
67    pub fn endpoint(mut self, endpoint: &str) -> Self {
68        if !endpoint.is_empty() {
69            self.config.endpoint = Some(endpoint.to_owned());
70        }
71        self
72    }
73
74    /// set the network address of redis cluster service.
75    /// This parameter is mutually exclusive with the endpoint parameter.
76    ///
77    /// currently supported schemes:
78    /// - no scheme: will be seen as "tcp"
79    /// - "tcp" or "redis": unsecured redis connections
80    /// - "rediss": secured redis connections
81    /// - "unix" or "redis+unix": unix socket connection
82    pub fn cluster_endpoints(mut self, cluster_endpoints: &str) -> Self {
83        if !cluster_endpoints.is_empty() {
84            self.config.cluster_endpoints = Some(cluster_endpoints.to_owned());
85        }
86        self
87    }
88
89    /// set the username for redis
90    ///
91    /// default: no username
92    pub fn username(mut self, username: &str) -> Self {
93        if !username.is_empty() {
94            self.config.username = Some(username.to_owned());
95        }
96        self
97    }
98
99    /// set the password for redis
100    ///
101    /// default: no password
102    pub fn password(mut self, password: &str) -> Self {
103        if !password.is_empty() {
104            self.config.password = Some(password.to_owned());
105        }
106        self
107    }
108
109    /// set the db used in redis
110    ///
111    /// default: 0
112    pub fn db(mut self, db: i64) -> Self {
113        self.config.db = db;
114        self
115    }
116
117    /// Set the default ttl for redis services.
118    ///
119    /// If set, we will specify `EX` for write operations.
120    pub fn default_ttl(mut self, ttl: Duration) -> Self {
121        self.config.default_ttl = Some(ttl);
122        self
123    }
124
125    /// set the working directory, all operations will be performed under it.
126    ///
127    /// default: "/"
128    pub fn root(mut self, root: &str) -> Self {
129        self.config.root = if root.is_empty() {
130            None
131        } else {
132            Some(root.to_string())
133        };
134
135        self
136    }
137}
138
139impl Builder for RedisBuilder {
140    type Config = RedisConfig;
141
142    fn build(self) -> Result<impl Access> {
143        let root = normalize_root(
144            self.config
145                .root
146                .clone()
147                .unwrap_or_else(|| "/".to_string())
148                .as_str(),
149        );
150
151        if let Some(endpoints) = self.config.cluster_endpoints.clone() {
152            let mut cluster_endpoints: Vec<ConnectionInfo> = Vec::default();
153            for endpoint in endpoints.split(',') {
154                cluster_endpoints.push(self.get_connection_info(endpoint.to_string())?);
155            }
156            let mut client_builder = ClusterClientBuilder::new(cluster_endpoints);
157            if let Some(username) = &self.config.username {
158                client_builder = client_builder.username(username.clone());
159            }
160            if let Some(password) = &self.config.password {
161                client_builder = client_builder.password(password.clone());
162            }
163            let client = client_builder.build().map_err(format_redis_error)?;
164
165            let conn = OnceCell::new();
166
167            Ok(RedisAccessor::new(RedisCore {
168                addr: endpoints,
169                client: None,
170                cluster_client: Some(client),
171                conn,
172                default_ttl: self.config.default_ttl,
173            })
174            .with_normalized_root(root))
175        } else {
176            let endpoint = self
177                .config
178                .endpoint
179                .clone()
180                .unwrap_or_else(|| DEFAULT_REDIS_ENDPOINT.to_string());
181
182            let client =
183                Client::open(self.get_connection_info(endpoint.clone())?).map_err(|e| {
184                    Error::new(ErrorKind::ConfigInvalid, "invalid or unsupported scheme")
185                        .with_context("service", Scheme::Redis)
186                        .with_context("endpoint", self.config.endpoint.as_ref().unwrap())
187                        .with_context("db", self.config.db.to_string())
188                        .set_source(e)
189                })?;
190
191            let conn = OnceCell::new();
192            Ok(RedisAccessor::new(RedisCore {
193                addr: endpoint,
194                client: Some(client),
195                cluster_client: None,
196                conn,
197                default_ttl: self.config.default_ttl,
198            })
199            .with_normalized_root(root))
200        }
201    }
202}
203
204impl RedisBuilder {
205    fn get_connection_info(&self, endpoint: String) -> Result<ConnectionInfo> {
206        let ep_url = endpoint.parse::<Uri>().map_err(|e| {
207            Error::new(ErrorKind::ConfigInvalid, "endpoint is invalid")
208                .with_context("service", Scheme::Redis)
209                .with_context("endpoint", endpoint)
210                .set_source(e)
211        })?;
212
213        let con_addr = match ep_url.scheme_str() {
214            Some("tcp") | Some("redis") | None => {
215                let host = ep_url
216                    .host()
217                    .map(|h| h.to_string())
218                    .unwrap_or_else(|| "127.0.0.1".to_string());
219                let port = ep_url.port_u16().unwrap_or(DEFAULT_REDIS_PORT);
220                ConnectionAddr::Tcp(host, port)
221            }
222            Some("rediss") => {
223                let host = ep_url
224                    .host()
225                    .map(|h| h.to_string())
226                    .unwrap_or_else(|| "127.0.0.1".to_string());
227                let port = ep_url.port_u16().unwrap_or(DEFAULT_REDIS_PORT);
228                ConnectionAddr::TcpTls {
229                    host,
230                    port,
231                    insecure: false,
232                    tls_params: None,
233                }
234            }
235            Some("unix") | Some("redis+unix") => {
236                let path = PathBuf::from(ep_url.path());
237                ConnectionAddr::Unix(path)
238            }
239            Some(s) => {
240                return Err(
241                    Error::new(ErrorKind::ConfigInvalid, "invalid or unsupported scheme")
242                        .with_context("service", Scheme::Redis)
243                        .with_context("scheme", s),
244                );
245            }
246        };
247
248        let redis_info = RedisConnectionInfo {
249            db: self.config.db,
250            username: self.config.username.clone(),
251            password: self.config.password.clone(),
252            protocol: ProtocolVersion::RESP2,
253        };
254
255        Ok(ConnectionInfo {
256            addr: con_addr,
257            redis: redis_info,
258        })
259    }
260}
261
262/// RedisAccessor implements Access trait directly
263#[derive(Debug, Clone)]
264pub struct RedisAccessor {
265    core: std::sync::Arc<RedisCore>,
266    root: String,
267    info: std::sync::Arc<AccessorInfo>,
268}
269
270impl RedisAccessor {
271    fn new(core: RedisCore) -> Self {
272        let info = AccessorInfo::default();
273        info.set_scheme(REDIS_SCHEME);
274        info.set_name(&core.addr);
275        info.set_root("/");
276        info.set_native_capability(Capability {
277            read: true,
278            write: true,
279            delete: true,
280            stat: true,
281            write_can_empty: true,
282            shared: true,
283            ..Default::default()
284        });
285
286        Self {
287            core: std::sync::Arc::new(core),
288            root: "/".to_string(),
289            info: std::sync::Arc::new(info),
290        }
291    }
292
293    fn with_normalized_root(mut self, root: String) -> Self {
294        self.info.set_root(&root);
295        self.root = root;
296        self
297    }
298}
299
300impl Access for RedisAccessor {
301    type Reader = Buffer;
302    type Writer = RedisWriter;
303    type Lister = ();
304    type Deleter = oio::OneShotDeleter<RedisDeleter>;
305
306    fn info(&self) -> std::sync::Arc<AccessorInfo> {
307        self.info.clone()
308    }
309
310    async fn stat(&self, path: &str, _: OpStat) -> Result<RpStat> {
311        let p = build_abs_path(&self.root, path);
312
313        if p == build_abs_path(&self.root, "") {
314            Ok(RpStat::new(Metadata::new(EntryMode::DIR)))
315        } else {
316            let bs = self.core.get(&p).await?;
317            match bs {
318                Some(bs) => Ok(RpStat::new(
319                    Metadata::new(EntryMode::FILE).with_content_length(bs.len() as u64),
320                )),
321                None => Err(Error::new(ErrorKind::NotFound, "key not found in redis")),
322            }
323        }
324    }
325
326    async fn read(&self, path: &str, args: OpRead) -> Result<(RpRead, Self::Reader)> {
327        let p = build_abs_path(&self.root, path);
328
329        let range = args.range();
330        let buffer = if range.is_full() {
331            // Full read - use GET
332            match self.core.get(&p).await? {
333                Some(bs) => bs,
334                None => return Err(Error::new(ErrorKind::NotFound, "key not found in redis")),
335            }
336        } else {
337            // Range read - use GETRANGE
338            let start = range.offset() as isize;
339            let end = match range.size() {
340                Some(size) => (range.offset() + size - 1) as isize,
341                None => -1, // Redis uses -1 for end of string
342            };
343
344            match self.core.get_range(&p, start, end).await? {
345                Some(bs) => bs,
346                None => return Err(Error::new(ErrorKind::NotFound, "key not found in redis")),
347            }
348        };
349
350        Ok((RpRead::new(), buffer))
351    }
352
353    async fn write(&self, path: &str, _: OpWrite) -> Result<(RpWrite, Self::Writer)> {
354        let p = build_abs_path(&self.root, path);
355        Ok((RpWrite::new(), RedisWriter::new(self.core.clone(), p)))
356    }
357
358    async fn delete(&self) -> Result<(RpDelete, Self::Deleter)> {
359        Ok((
360            RpDelete::default(),
361            oio::OneShotDeleter::new(RedisDeleter::new(self.core.clone(), self.root.clone())),
362        ))
363    }
364
365    async fn list(&self, path: &str, _: OpList) -> Result<(RpList, Self::Lister)> {
366        let _ = build_abs_path(&self.root, path);
367        // Redis doesn't support listing keys, return empty list
368        Ok((RpList::default(), ()))
369    }
370}
```
