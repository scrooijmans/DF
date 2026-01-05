# 

opendal/services/memcached/

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
18use std::sync::Arc;
19use std::time::Duration;
20
21use tokio::sync::OnceCell;
22
23use super::config::MemcachedConfig;
24use super::core::*;
25use super::deleter::MemcachedDeleter;
26use super::writer::MemcachedWriter;
27use crate::raw::*;
28use crate::*;
29
30/// [Memcached](https://memcached.org/) service support.
31#[doc = include_str!("docs.md")]
32#[derive(Clone, Default)]
33pub struct MemcachedBuilder {
34    pub(super) config: MemcachedConfig,
35}
36
37impl MemcachedBuilder {
38    /// set the network address of memcached service.
39    ///
40    /// For example: "tcp://localhost:11211"
41    pub fn endpoint(mut self, endpoint: &str) -> Self {
42        if !endpoint.is_empty() {
43            self.config.endpoint = Some(endpoint.to_owned());
44        }
45        self
46    }
47
48    /// set the working directory, all operations will be performed under it.
49    ///
50    /// default: "/"
51    pub fn root(mut self, root: &str) -> Self {
52        self.config.root = if root.is_empty() {
53            None
54        } else {
55            Some(root.to_string())
56        };
57
58        self
59    }
60
61    /// set the username.
62    pub fn username(mut self, username: &str) -> Self {
63        self.config.username = Some(username.to_string());
64        self
65    }
66
67    /// set the password.
68    pub fn password(mut self, password: &str) -> Self {
69        self.config.password = Some(password.to_string());
70        self
71    }
72
73    /// Set the default ttl for memcached services.
74    pub fn default_ttl(mut self, ttl: Duration) -> Self {
75        self.config.default_ttl = Some(ttl);
76        self
77    }
78}
79
80impl Builder for MemcachedBuilder {
81    type Config = MemcachedConfig;
82
83    fn build(self) -> Result<impl Access> {
84        let endpoint = self.config.endpoint.clone().ok_or_else(|| {
85            Error::new(ErrorKind::ConfigInvalid, "endpoint is empty")
86                .with_context("service", Scheme::Memcached)
87        })?;
88        let uri = http::Uri::try_from(&endpoint).map_err(|err| {
89            Error::new(ErrorKind::ConfigInvalid, "endpoint is invalid")
90                .with_context("service", Scheme::Memcached)
91                .with_context("endpoint", &endpoint)
92                .set_source(err)
93        })?;
94
95        match uri.scheme_str() {
96            // If scheme is none, we will use tcp by default.
97            None => (),
98            Some(scheme) => {
99                // We only support tcp by now.
100                if scheme != "tcp" {
101                    return Err(Error::new(
102                        ErrorKind::ConfigInvalid,
103                        "endpoint is using invalid scheme",
104                    )
105                    .with_context("service", Scheme::Memcached)
106                    .with_context("endpoint", &endpoint)
107                    .with_context("scheme", scheme.to_string()));
108                }
109            }
110        };
111
112        let host = if let Some(host) = uri.host() {
113            host.to_string()
114        } else {
115            return Err(
116                Error::new(ErrorKind::ConfigInvalid, "endpoint doesn't have host")
117                    .with_context("service", Scheme::Memcached)
118                    .with_context("endpoint", &endpoint),
119            );
120        };
121        let port = if let Some(port) = uri.port_u16() {
122            port
123        } else {
124            return Err(
125                Error::new(ErrorKind::ConfigInvalid, "endpoint doesn't have port")
126                    .with_context("service", Scheme::Memcached)
127                    .with_context("endpoint", &endpoint),
128            );
129        };
130        let endpoint = format!("{host}:{port}",);
131
132        let root = normalize_root(
133            self.config
134                .root
135                .clone()
136                .unwrap_or_else(|| "/".to_string())
137                .as_str(),
138        );
139
140        let conn = OnceCell::new();
141        Ok(MemcachedBackend::new(MemcachedCore {
142            conn,
143            endpoint,
144            username: self.config.username.clone(),
145            password: self.config.password.clone(),
146            default_ttl: self.config.default_ttl,
147        })
148        .with_normalized_root(root))
149    }
150}
151
152/// Backend for memcached services.
153#[derive(Clone, Debug)]
154pub struct MemcachedBackend {
155    core: Arc<MemcachedCore>,
156    root: String,
157    info: Arc<AccessorInfo>,
158}
159
160impl MemcachedBackend {
161    pub fn new(core: MemcachedCore) -> Self {
162        let info = AccessorInfo::default();
163        info.set_scheme(Scheme::Memcached.into_static());
164        info.set_name("memcached");
165        info.set_root("/");
166        info.set_native_capability(Capability {
167            read: true,
168            stat: true,
169            write: true,
170            write_can_empty: true,
171            delete: true,
172            shared: true,
173            ..Default::default()
174        });
175
176        Self {
177            core: Arc::new(core),
178            root: "/".to_string(),
179            info: Arc::new(info),
180        }
181    }
182
183    fn with_normalized_root(mut self, root: String) -> Self {
184        self.info.set_root(&root);
185        self.root = root;
186        self
187    }
188}
189
190impl Access for MemcachedBackend {
191    type Reader = Buffer;
192    type Writer = MemcachedWriter;
193    type Lister = ();
194    type Deleter = oio::OneShotDeleter<MemcachedDeleter>;
195
196    fn info(&self) -> Arc<AccessorInfo> {
197        self.info.clone()
198    }
199
200    async fn stat(&self, path: &str, _: OpStat) -> Result<RpStat> {
201        let p = build_abs_path(&self.root, path);
202
203        if p == build_abs_path(&self.root, "") {
204            Ok(RpStat::new(Metadata::new(EntryMode::DIR)))
205        } else {
206            let bs = self.core.get(&p).await?;
207            match bs {
208                Some(bs) => Ok(RpStat::new(
209                    Metadata::new(EntryMode::FILE).with_content_length(bs.len() as u64),
210                )),
211                None => Err(Error::new(ErrorKind::NotFound, "kv not found in memcached")),
212            }
213        }
214    }
215
216    async fn read(&self, path: &str, args: OpRead) -> Result<(RpRead, Self::Reader)> {
217        let p = build_abs_path(&self.root, path);
218        let bs = match self.core.get(&p).await? {
219            Some(bs) => bs,
220            None => return Err(Error::new(ErrorKind::NotFound, "kv not found in memcached")),
221        };
222        Ok((RpRead::new(), bs.slice(args.range().to_range_as_usize())))
223    }
224
225    async fn write(&self, path: &str, _: OpWrite) -> Result<(RpWrite, Self::Writer)> {
226        let p = build_abs_path(&self.root, path);
227        Ok((RpWrite::new(), MemcachedWriter::new(self.core.clone(), p)))
228    }
229
230    async fn delete(&self) -> Result<(RpDelete, Self::Deleter)> {
231        Ok((
232            RpDelete::default(),
233            oio::OneShotDeleter::new(MemcachedDeleter::new(self.core.clone(), self.root.clone())),
234        ))
235    }
236
237    async fn list(&self, path: &str, _: OpList) -> Result<(RpList, Self::Lister)> {
238        let _ = build_abs_path(&self.root, path);
239        Ok((RpList::default(), ()))
240    }
241}
```
