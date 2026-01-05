# 

opendal/services/moka/

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
21use std::time::Duration;
22
23use log::debug;
24
25use super::MOKA_SCHEME;
26use super::core::*;
27use super::delete::MokaDeleter;
28use super::lister::MokaLister;
29use super::writer::MokaWriter;
30use crate::raw::oio;
31use crate::raw::signed_to_duration;
32use crate::raw::*;
33use crate::services::MokaConfig;
34use crate::*;
35
36/// Type alias of [`moka::future::Cache`](https://docs.rs/moka/latest/moka/future/struct.Cache.html)
37pub type MokaCache<K, V> = moka::future::Cache<K, V>;
38/// Type alias of [`moka::future::CacheBuilder`](https://docs.rs/moka/latest/moka/future/struct.CacheBuilder.html)
39pub type MokaCacheBuilder<K, V> = moka::future::CacheBuilder<K, V, MokaCache<K, V>>;
40
41/// [moka](https://github.com/moka-rs/moka) backend support.
42#[doc = include_str!("docs.md")]
43#[derive(Default)]
44pub struct MokaBuilder {
45    pub(super) config: MokaConfig,
46    pub(super) builder: MokaCacheBuilder<String, MokaValue>,
47}
48
49impl Debug for MokaBuilder {
50    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
51        f.debug_struct("MokaBuilder")
52            .field("config", &self.config)
53            .finish()
54    }
55}
56
57impl MokaBuilder {
58    /// Create a [`MokaBuilder`] with the given [`moka::future::CacheBuilder`].
59    ///
60    /// Refer to [`moka::future::CacheBuilder`](https://docs.rs/moka/latest/moka/future/struct.CacheBuilder.html)
61    ///
62    /// # Example
63    ///
64    /// ```no_run
65    /// # use std::sync::Arc;
66    /// # use std::time::Duration;
67    /// # use log::debug;
68    /// # use moka::notification::RemovalCause;
69    /// # use opendal::services::Moka;
70    /// # use opendal::services::MokaCacheBuilder;
71    /// # use opendal::services::MokaValue;
72    /// # use opendal::Configurator;
73    /// let moka = Moka::new(
74    ///     MokaCacheBuilder::<String, MokaValue>::default()
75    ///         .name("demo")
76    ///         .max_capacity(1000)
77    ///         .time_to_live(Duration::from_secs(300))
78    ///         .weigher(|k, v| (k.len() + v.content.len()) as u32)
79    ///         .eviction_listener(|k: Arc<String>, v: MokaValue, cause: RemovalCause| {
80    ///             debug!(
81    ///                 "moka cache eviction listener, key = {}, value = {:?}, cause = {:?}",
82    ///                 k.as_str(), v.content.to_vec(), cause
83    ///             );
84    ///         })
85    /// );
86    /// ```
87    pub fn new(builder: MokaCacheBuilder<String, MokaValue>) -> Self {
88        Self {
89            builder,
90            ..Default::default()
91        }
92    }
93
94    /// Sets the name of the cache.
95    ///
96    /// Refer to [`moka::future::CacheBuilder::name`](https://docs.rs/moka/latest/moka/future/struct.CacheBuilder.html#method.name)
97    pub fn name(mut self, v: &str) -> Self {
98        if !v.is_empty() {
99            self.config.name = Some(v.to_owned());
100        }
101        self
102    }
103
104    /// Sets the max capacity of the cache.
105    ///
106    /// Refer to [`moka::future::CacheBuilder::max_capacity`](https://docs.rs/moka/latest/moka/future/struct.CacheBuilder.html#method.max_capacity)
107    pub fn max_capacity(mut self, v: u64) -> Self {
108        if v != 0 {
109            self.config.max_capacity = Some(v);
110        }
111        self
112    }
113
114    /// Sets the time to live of the cache.
115    ///
116    /// Refer to [`moka::future::CacheBuilder::time_to_live`](https://docs.rs/moka/latest/moka/future/struct.CacheBuilder.html#method.time_to_live)
117    pub fn time_to_live(mut self, v: Duration) -> Self {
118        if !v.is_zero() {
119            self.config.time_to_live = Some(format!("{}s", v.as_secs()));
120        }
121        self
122    }
123
124    /// Sets the time to idle of the cache.
125    ///
126    /// Refer to [`moka::future::CacheBuilder::time_to_idle`](https://docs.rs/moka/latest/moka/sync/struct.CacheBuilder.html#method.time_to_idle)
127    pub fn time_to_idle(mut self, v: Duration) -> Self {
128        if !v.is_zero() {
129            self.config.time_to_idle = Some(format!("{}s", v.as_secs()));
130        }
131        self
132    }
133
134    /// Set the root path of this backend
135    pub fn root(mut self, path: &str) -> Self {
136        self.config.root = if path.is_empty() {
137            None
138        } else {
139            Some(path.to_string())
140        };
141        self
142    }
143}
144
145impl Builder for MokaBuilder {
146    type Config = MokaConfig;
147
148    fn build(self) -> Result<impl Access> {
149        debug!("backend build started: {:?}", &self);
150
151        let root = normalize_root(
152            self.config
153                .root
154                .clone()
155                .unwrap_or_else(|| "/".to_string())
156                .as_str(),
157        );
158
159        let mut builder = self.builder;
160
161        if let Some(v) = &self.config.name {
162            builder = builder.name(v);
163        }
164        if let Some(v) = self.config.max_capacity {
165            builder = builder.max_capacity(v);
166        }
167        if let Some(value) = self.config.time_to_live.as_deref() {
168            let duration = signed_to_duration(value)?;
169            builder = builder.time_to_live(duration);
170        }
171        if let Some(value) = self.config.time_to_idle.as_deref() {
172            let duration = signed_to_duration(value)?;
173            builder = builder.time_to_idle(duration);
174        }
175
176        debug!("backend build finished: {:?}", self.config);
177
178        let core = MokaCore {
179            cache: builder.build(),
180        };
181
182        Ok(MokaAccessor::new(core).with_normalized_root(root))
183    }
184}
185
186/// MokaAccessor implements Access trait directly
187#[derive(Debug, Clone)]
188pub struct MokaAccessor {
189    core: Arc<MokaCore>,
190    root: String,
191    info: Arc<AccessorInfo>,
192}
193
194impl MokaAccessor {
195    fn new(core: MokaCore) -> Self {
196        let info = AccessorInfo::default();
197        info.set_scheme(MOKA_SCHEME);
198        info.set_name(core.cache.name().unwrap_or("moka"));
199        info.set_root("/");
200        info.set_native_capability(Capability {
201            read: true,
202            write: true,
203            write_can_empty: true,
204            write_with_cache_control: true,
205            write_with_content_type: true,
206            write_with_content_disposition: true,
207            write_with_content_encoding: true,
208            delete: true,
209            stat: true,
210            list: true,
211            shared: false,
212            ..Default::default()
213        });
214
215        Self {
216            core: Arc::new(core),
217            root: "/".to_string(),
218            info: Arc::new(info),
219        }
220    }
221
222    fn with_normalized_root(mut self, root: String) -> Self {
223        self.info.set_root(&root);
224        self.root = root;
225        self
226    }
227}
228
229impl Access for MokaAccessor {
230    type Reader = Buffer;
231    type Writer = MokaWriter;
232    type Lister = oio::HierarchyLister<MokaLister>;
233    type Deleter = oio::OneShotDeleter<MokaDeleter>;
234
235    fn info(&self) -> Arc<AccessorInfo> {
236        self.info.clone()
237    }
238
239    async fn stat(&self, path: &str, _: OpStat) -> Result<RpStat> {
240        let p = build_abs_path(&self.root, path);
241
242        if p == build_abs_path(&self.root, "") {
243            Ok(RpStat::new(Metadata::new(EntryMode::DIR)))
244        } else {
245            // Check the exact path first
246            match self.core.get(&p).await? {
247                Some(value) => {
248                    // Use the stored metadata but override mode if necessary
249                    let mut metadata = value.metadata.clone();
250                    // If path ends with '/' but we found a file, return DIR
251                    // This is because CompleteLayer's create_dir creates empty files with '/' suffix
252                    if p.ends_with('/') && metadata.mode() != EntryMode::DIR {
253                        metadata.set_mode(EntryMode::DIR);
254                    }
255                    Ok(RpStat::new(metadata))
256                }
257                None => {
258                    // If path ends with '/', check if there are any children
259                    if p.ends_with('/') {
260                        let has_children = self
261                            .core
262                            .cache
263                            .iter()
264                            .any(|kv| kv.0.starts_with(&p) && kv.0.len() > p.len());
265
266                        if has_children {
267                            Ok(RpStat::new(Metadata::new(EntryMode::DIR)))
268                        } else {
269                            Err(Error::new(ErrorKind::NotFound, "key not found in moka"))
270                        }
271                    } else {
272                        Err(Error::new(ErrorKind::NotFound, "key not found in moka"))
273                    }
274                }
275            }
276        }
277    }
278
279    async fn read(&self, path: &str, args: OpRead) -> Result<(RpRead, Self::Reader)> {
280        let p = build_abs_path(&self.root, path);
281
282        match self.core.get(&p).await? {
283            Some(value) => {
284                let buffer = if args.range().is_full() {
285                    value.content
286                } else {
287                    let range = args.range();
288                    let start = range.offset() as usize;
289                    let end = match range.size() {
290                        Some(size) => (range.offset() + size) as usize,
291                        None => value.content.len(),
292                    };
293                    value.content.slice(start..end.min(value.content.len()))
294                };
295                Ok((RpRead::new(), buffer))
296            }
297            None => Err(Error::new(ErrorKind::NotFound, "key not found in moka")),
298        }
299    }
300
301    async fn write(&self, path: &str, args: OpWrite) -> Result<(RpWrite, Self::Writer)> {
302        let p = build_abs_path(&self.root, path);
303        Ok((RpWrite::new(), MokaWriter::new(self.core.clone(), p, args)))
304    }
305
306    async fn delete(&self) -> Result<(RpDelete, Self::Deleter)> {
307        Ok((
308            RpDelete::default(),
309            oio::OneShotDeleter::new(MokaDeleter::new(self.core.clone(), self.root.clone())),
310        ))
311    }
312
313    async fn list(&self, path: &str, args: OpList) -> Result<(RpList, Self::Lister)> {
314        // For moka, we don't distinguish between files and directories
315        // Just return the lister to iterate through all matching keys
316        let lister = MokaLister::new(self.core.clone(), self.root.clone(), path.to_string());
317        let lister = oio::HierarchyLister::new(lister, path, args.recursive());
318        Ok((RpList::default(), lister))
319    }
320}
```
