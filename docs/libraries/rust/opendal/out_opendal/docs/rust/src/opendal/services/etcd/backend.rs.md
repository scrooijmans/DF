# 

opendal/services/etcd/

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
22use etcd_client::Certificate;
23use etcd_client::ConnectOptions;
24use etcd_client::Identity;
25use etcd_client::TlsOptions;
26use tokio::sync::OnceCell;
27
28use super::ETCD_SCHEME;
29use super::core::EtcdCore;
30use super::core::constants::DEFAULT_ETCD_ENDPOINTS;
31use super::deleter::EtcdDeleter;
32use super::lister::EtcdLister;
33use super::writer::EtcdWriter;
34use crate::raw::*;
35use crate::services::EtcdConfig;
36use crate::*;
37
38/// [Etcd](https://etcd.io/) services support.
39#[doc = include_str!("docs.md")]
40#[derive(Clone, Default)]
41pub struct EtcdBuilder {
42    pub(super) config: EtcdConfig,
43}
44
45impl Debug for EtcdBuilder {
46    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
47        let mut ds = f.debug_struct("Builder");
48
49        ds.field("config", &self.config);
50        ds.finish()
51    }
52}
53
54impl EtcdBuilder {
55    /// set the network address of etcd service.
56    ///
57    /// default: "http://127.0.0.1:2379"
58    pub fn endpoints(mut self, endpoints: &str) -> Self {
59        if !endpoints.is_empty() {
60            self.config.endpoints = Some(endpoints.to_owned());
61        }
62        self
63    }
64
65    /// set the username for etcd
66    ///
67    /// default: no username
68    pub fn username(mut self, username: &str) -> Self {
69        if !username.is_empty() {
70            self.config.username = Some(username.to_owned());
71        }
72        self
73    }
74
75    /// set the password for etcd
76    ///
77    /// default: no password
78    pub fn password(mut self, password: &str) -> Self {
79        if !password.is_empty() {
80            self.config.password = Some(password.to_owned());
81        }
82        self
83    }
84
85    /// set the working directory, all operations will be performed under it.
86    ///
87    /// default: "/"
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
98    /// Set the certificate authority file path.
99    ///
100    /// default is None
101    pub fn ca_path(mut self, ca_path: &str) -> Self {
102        if !ca_path.is_empty() {
103            self.config.ca_path = Some(ca_path.to_string())
104        }
105        self
106    }
107
108    /// Set the certificate file path.
109    ///
110    /// default is None
111    pub fn cert_path(mut self, cert_path: &str) -> Self {
112        if !cert_path.is_empty() {
113            self.config.cert_path = Some(cert_path.to_string())
114        }
115        self
116    }
117
118    /// Set the key file path.
119    ///
120    /// default is None
121    pub fn key_path(mut self, key_path: &str) -> Self {
122        if !key_path.is_empty() {
123            self.config.key_path = Some(key_path.to_string())
124        }
125        self
126    }
127}
128
129impl Builder for EtcdBuilder {
130    type Config = EtcdConfig;
131
132    fn build(self) -> Result<impl Access> {
133        let endpoints = self
134            .config
135            .endpoints
136            .clone()
137            .unwrap_or_else(|| DEFAULT_ETCD_ENDPOINTS.to_string());
138
139        let endpoints: Vec<String> = endpoints.split(',').map(|s| s.to_string()).collect();
140
141        let mut options = ConnectOptions::new();
142
143        if self.config.ca_path.is_some()
144            && self.config.cert_path.is_some()
145            && self.config.key_path.is_some()
146        {
147            let ca = self.load_pem(self.config.ca_path.clone().unwrap().as_str())?;
148            let key = self.load_pem(self.config.key_path.clone().unwrap().as_str())?;
149            let cert = self.load_pem(self.config.cert_path.clone().unwrap().as_str())?;
150
151            let tls_options = TlsOptions::default()
152                .ca_certificate(Certificate::from_pem(ca))
153                .identity(Identity::from_pem(cert, key));
154            options = options.with_tls(tls_options);
155        }
156
157        if let Some(username) = self.config.username.clone() {
158            options = options.with_user(
159                username,
160                self.config.password.clone().unwrap_or("".to_string()),
161            );
162        }
163
164        let root = normalize_root(
165            self.config
166                .root
167                .clone()
168                .unwrap_or_else(|| "/".to_string())
169                .as_str(),
170        );
171
172        let client = OnceCell::new();
173
174        let core = EtcdCore {
175            endpoints,
176            client,
177            options,
178        };
179
180        Ok(EtcdAccessor::new(core, &root))
181    }
182}
183
184impl EtcdBuilder {
185    fn load_pem(&self, path: &str) -> Result<String> {
186        std::fs::read_to_string(path)
187            .map_err(|err| Error::new(ErrorKind::Unexpected, "invalid file path").set_source(err))
188    }
189}
190
191#[derive(Debug, Clone)]
192pub struct EtcdAccessor {
193    core: Arc<EtcdCore>,
194    info: Arc<AccessorInfo>,
195}
196
197impl EtcdAccessor {
198    fn new(core: EtcdCore, root: &str) -> Self {
199        let info = AccessorInfo::default();
200        info.set_scheme(ETCD_SCHEME);
201        info.set_name("etcd");
202        info.set_root(root);
203        info.set_native_capability(Capability {
204            read: true,
205
206            write: true,
207            write_can_empty: true,
208
209            delete: true,
210            stat: true,
211            list: true,
212
213            shared: true,
214
215            ..Default::default()
216        });
217
218        Self {
219            core: Arc::new(core),
220            info: Arc::new(info),
221        }
222    }
223}
224
225impl Access for EtcdAccessor {
226    type Reader = Buffer;
227    type Writer = EtcdWriter;
228    type Lister = oio::HierarchyLister<EtcdLister>;
229    type Deleter = oio::OneShotDeleter<EtcdDeleter>;
230
231    fn info(&self) -> Arc<AccessorInfo> {
232        self.info.clone()
233    }
234
235    async fn create_dir(&self, path: &str, _args: OpCreateDir) -> Result<RpCreateDir> {
236        let abs_path = build_abs_path(&self.info.root(), path);
237
238        // In etcd, we simulate directory creation by storing an empty value
239        // with the directory path (ensuring it ends with '/')
240        let dir_path = if abs_path.ends_with('/') {
241            abs_path
242        } else {
243            format!("{abs_path}/")
244        };
245
246        // Store an empty buffer to represent the directory
247        self.core.set(&dir_path, Buffer::new()).await?;
248
249        Ok(RpCreateDir::default())
250    }
251
252    async fn stat(&self, path: &str, _: OpStat) -> Result<RpStat> {
253        let abs_path = build_abs_path(&self.info.root(), path);
254
255        // First check if it's a direct key
256        match self.core.get(&abs_path).await? {
257            Some(buffer) => {
258                let mut metadata = Metadata::new(EntryMode::from_path(&abs_path));
259                metadata.set_content_length(buffer.len() as u64);
260                Ok(RpStat::new(metadata))
261            }
262            None => {
263                // Check if it's a directory by looking for keys with this prefix
264                let prefix = if abs_path.ends_with('/') {
265                    abs_path
266                } else {
267                    format!("{abs_path}/")
268                };
269
270                // Use etcd prefix query to check if any keys exist with this prefix
271                let has_children = self.core.has_prefix(&prefix).await?;
272                if has_children {
273                    // Has children, it's a directory
274                    let metadata = Metadata::new(EntryMode::DIR);
275                    Ok(RpStat::new(metadata))
276                } else {
277                    Err(Error::new(ErrorKind::NotFound, "path not found"))
278                }
279            }
280        }
281    }
282
283    async fn read(&self, path: &str, op: OpRead) -> Result<(RpRead, Self::Reader)> {
284        let abs_path = build_abs_path(&self.info.root(), path);
285
286        match self.core.get(&abs_path).await? {
287            Some(buffer) => {
288                let range = op.range();
289
290                // If range is full, return the buffer directly
291                if range.is_full() {
292                    return Ok((RpRead::new(), buffer));
293                }
294
295                // Handle range requests
296                let offset = range.offset() as usize;
297                if offset >= buffer.len() {
298                    return Err(Error::new(
299                        ErrorKind::RangeNotSatisfied,
300                        "range start offset exceeds content length",
301                    ));
302                }
303
304                let size = range.size().map(|s| s as usize);
305                let end = size.map_or(buffer.len(), |s| (offset + s).min(buffer.len()));
306                let sliced_buffer = buffer.slice(offset..end);
307
308                Ok((RpRead::new(), sliced_buffer))
309            }
310            None => Err(Error::new(ErrorKind::NotFound, "path not found")),
311        }
312    }
313
314    async fn write(&self, path: &str, _op: OpWrite) -> Result<(RpWrite, Self::Writer)> {
315        let abs_path = build_abs_path(&self.info.root(), path);
316        let writer = EtcdWriter::new(self.core.clone(), abs_path);
317        Ok((RpWrite::new(), writer))
318    }
319
320    async fn delete(&self) -> Result<(RpDelete, Self::Deleter)> {
321        let deleter = oio::OneShotDeleter::new(EtcdDeleter::new(
322            self.core.clone(),
323            self.info.root().to_string(),
324        ));
325        Ok((RpDelete::default(), deleter))
326    }
327
328    async fn list(&self, path: &str, args: OpList) -> Result<(RpList, Self::Lister)> {
329        let lister = EtcdLister::new(
330            self.core.clone(),
331            self.info.root().to_string(),
332            path.to_string(),
333        )
334        .await?;
335        let lister = oio::HierarchyLister::new(lister, path, args.recursive());
336        Ok((RpList::default(), lister))
337    }
338}
```
