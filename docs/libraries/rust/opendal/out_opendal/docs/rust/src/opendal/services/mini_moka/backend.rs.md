# 

opendal/services/mini_moka/

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
25use super::MINI_MOKA_SCHEME;
26use super::core::*;
27use super::delete::MiniMokaDeleter;
28use super::lister::MiniMokaLister;
29use super::writer::MiniMokaWriter;
30use crate::raw::oio;
31use crate::raw::oio::HierarchyLister;
32use crate::raw::signed_to_duration;
33use crate::raw::*;
34use crate::services::MiniMokaConfig;
35use crate::*;
36
37/// [mini-moka](https://github.com/moka-rs/mini-moka) backend support.
38#[doc = include_str!("docs.md")]
39#[derive(Default)]
40pub struct MiniMokaBuilder {
41    pub(super) config: MiniMokaConfig,
42}
43
44impl Debug for MiniMokaBuilder {
45    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
46        f.debug_struct("MiniMokaBuilder")
47            .field("config", &self.config)
48            .finish()
49    }
50}
51
52impl MiniMokaBuilder {
53    /// Create a [`MiniMokaBuilder`] with default configuration.
54    pub fn new() -> Self {
55        Self::default()
56    }
57
58    /// Sets the max capacity of the cache.
59    ///
60    /// Refer to [`mini-moka::sync::CacheBuilder::max_capacity`](https://docs.rs/mini-moka/latest/mini_moka/sync/struct.CacheBuilder.html#method.max_capacity)
61    pub fn max_capacity(mut self, v: u64) -> Self {
62        if v != 0 {
63            self.config.max_capacity = Some(v);
64        }
65        self
66    }
67
68    /// Sets the time to live of the cache.
69    ///
70    /// Refer to [`mini-moka::sync::CacheBuilder::time_to_live`](https://docs.rs/mini-moka/latest/mini_moka/sync/struct.CacheBuilder.html#method.time_to_live)
71    pub fn time_to_live(mut self, v: Duration) -> Self {
72        if !v.is_zero() {
73            self.config.time_to_live = Some(format!("{}s", v.as_secs()));
74        }
75        self
76    }
77
78    /// Sets the time to idle of the cache.
79    ///
80    /// Refer to [`mini-moka::sync::CacheBuilder::time_to_idle`](https://docs.rs/mini-moka/latest/mini_moka/sync/struct.CacheBuilder.html#method.time_to_idle)
81    pub fn time_to_idle(mut self, v: Duration) -> Self {
82        if !v.is_zero() {
83            self.config.time_to_idle = Some(format!("{}s", v.as_secs()));
84        }
85        self
86    }
87
88    /// Set root path of this backend
89    pub fn root(mut self, path: &str) -> Self {
90        self.config.root = if path.is_empty() {
91            None
92        } else {
93            Some(path.to_string())
94        };
95
96        self
97    }
98}
99
100impl Builder for MiniMokaBuilder {
101    type Config = MiniMokaConfig;
102
103    fn build(self) -> Result<impl Access> {
104        debug!("backend build started: {:?}", &self);
105
106        let mut builder: mini_moka::sync::CacheBuilder<String, MiniMokaValue, _> =
107            mini_moka::sync::Cache::builder();
108
109        // Use entries' bytes as capacity weigher.
110        builder = builder.weigher(|k, v| (k.len() + v.content.len()) as u32);
111
112        if let Some(v) = self.config.max_capacity {
113            builder = builder.max_capacity(v);
114        }
115        if let Some(value) = self.config.time_to_live.as_deref() {
116            let duration = signed_to_duration(value)?;
117            builder = builder.time_to_live(duration);
118        }
119        if let Some(value) = self.config.time_to_idle.as_deref() {
120            let duration = signed_to_duration(value)?;
121            builder = builder.time_to_idle(duration);
122        }
123
124        let cache = builder.build();
125
126        let root = normalize_root(self.config.root.as_deref().unwrap_or("/"));
127
128        let core = Arc::new(MiniMokaCore { cache });
129
130        debug!("backend build finished: {root}");
131        Ok(MiniMokaBackend::new(core, root))
132    }
133}
134
135#[derive(Debug)]
136struct MiniMokaBackend {
137    core: Arc<MiniMokaCore>,
138    root: String,
139}
140
141impl MiniMokaBackend {
142    fn new(core: Arc<MiniMokaCore>, root: String) -> Self {
143        Self { core, root }
144    }
145}
146
147impl Access for MiniMokaBackend {
148    type Reader = Buffer;
149    type Writer = MiniMokaWriter;
150    type Lister = HierarchyLister<MiniMokaLister>;
151    type Deleter = oio::OneShotDeleter<MiniMokaDeleter>;
152
153    fn info(&self) -> Arc<AccessorInfo> {
154        let info = AccessorInfo::default();
155        info.set_scheme(MINI_MOKA_SCHEME)
156            .set_root(&self.root)
157            .set_native_capability(Capability {
158                stat: true,
159                read: true,
160                write: true,
161                write_can_empty: true,
162                delete: true,
163                list: true,
164
165                ..Default::default()
166            });
167
168        Arc::new(info)
169    }
170
171    async fn stat(&self, path: &str, _: OpStat) -> Result<RpStat> {
172        let p = build_abs_path(&self.root, path);
173
174        // Check if path exists directly in cache
175        match self.core.get(&p) {
176            Some(value) => {
177                let mut metadata = value.metadata.clone();
178                if p.ends_with('/') {
179                    metadata.set_mode(EntryMode::DIR);
180                } else {
181                    metadata.set_mode(EntryMode::FILE);
182                }
183                Ok(RpStat::new(metadata))
184            }
185            None => {
186                if p.ends_with('/') {
187                    let is_prefix = self
188                        .core
189                        .cache
190                        .iter()
191                        .any(|entry| entry.key().starts_with(&p) && entry.key() != &p);
192
193                    if is_prefix {
194                        let mut metadata = Metadata::default();
195                        metadata.set_mode(EntryMode::DIR);
196                        return Ok(RpStat::new(metadata));
197                    }
198                }
199
200                Err(Error::new(ErrorKind::NotFound, "path not found"))
201            }
202        }
203    }
204
205    async fn read(&self, path: &str, op: OpRead) -> Result<(RpRead, Self::Reader)> {
206        let p = build_abs_path(&self.root, path);
207
208        match self.core.get(&p) {
209            Some(value) => {
210                let range = op.range();
211
212                // If range is full, return the content buffer directly
213                if range.is_full() {
214                    return Ok((RpRead::new(), value.content));
215                }
216
217                let offset = range.offset() as usize;
218                if offset >= value.content.len() {
219                    return Err(Error::new(
220                        ErrorKind::RangeNotSatisfied,
221                        "range start offset exceeds content length",
222                    ));
223                }
224
225                let size = range.size().map(|s| s as usize);
226                let end = size.map_or(value.content.len(), |s| {
227                    (offset + s).min(value.content.len())
228                });
229                let sliced_content = value.content.slice(offset..end);
230
231                Ok((RpRead::new(), sliced_content))
232            }
233            None => Err(Error::new(ErrorKind::NotFound, "path not found")),
234        }
235    }
236
237    async fn write(&self, path: &str, op: OpWrite) -> Result<(RpWrite, Self::Writer)> {
238        let p = build_abs_path(&self.root, path);
239        let writer = MiniMokaWriter::new(self.core.clone(), p, op);
240        Ok((RpWrite::new(), writer))
241    }
242
243    async fn delete(&self) -> Result<(RpDelete, Self::Deleter)> {
244        let deleter =
245            oio::OneShotDeleter::new(MiniMokaDeleter::new(self.core.clone(), self.root.clone()));
246        Ok((RpDelete::default(), deleter))
247    }
248
249    async fn list(&self, path: &str, op: OpList) -> Result<(RpList, Self::Lister)> {
250        let p = build_abs_path(&self.root, path);
251
252        let mini_moka_lister = MiniMokaLister::new(self.core.clone(), self.root.clone(), p);
253        let lister = HierarchyLister::new(mini_moka_lister, path, op.recursive());
254
255        Ok((RpList::default(), lister))
256    }
257}
```
