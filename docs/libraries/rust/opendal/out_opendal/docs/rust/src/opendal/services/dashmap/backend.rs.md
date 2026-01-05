# 

opendal/services/dashmap/

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
22use dashmap::DashMap;
23use log::debug;
24
25use super::DASHMAP_SCHEME;
26use super::core::DashmapCore;
27use super::delete::DashmapDeleter;
28use super::lister::DashmapLister;
29use super::writer::DashmapWriter;
30use crate::raw::oio;
31use crate::raw::*;
32use crate::services::DashmapConfig;
33use crate::*;
34
35/// [dashmap](https://github.com/xacrimon/dashmap) backend support.
36#[doc = include_str!("docs.md")]
37#[derive(Default)]
38pub struct DashmapBuilder {
39    pub(super) config: DashmapConfig,
40}
41
42impl Debug for DashmapBuilder {
43    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
44        f.debug_struct("DashmapBuilder")
45            .field("config", &self.config)
46            .finish()
47    }
48}
49
50impl DashmapBuilder {
51    /// Set the root for dashmap.
52    pub fn root(mut self, path: &str) -> Self {
53        self.config.root = if path.is_empty() {
54            None
55        } else {
56            Some(path.to_string())
57        };
58
59        self
60    }
61}
62
63impl Builder for DashmapBuilder {
64    type Config = DashmapConfig;
65
66    fn build(self) -> Result<impl Access> {
67        debug!("backend build started: {:?}", &self);
68
69        let root = normalize_root(
70            self.config
71                .root
72                .clone()
73                .unwrap_or_else(|| "/".to_string())
74                .as_str(),
75        );
76
77        debug!("backend build finished: {:?}", self.config);
78
79        let core = DashmapCore {
80            cache: DashMap::new(),
81        };
82
83        Ok(DashmapAccessor::new(core, root))
84    }
85}
86
87#[derive(Debug, Clone)]
88pub struct DashmapAccessor {
89    core: Arc<DashmapCore>,
90    root: String,
91    info: Arc<AccessorInfo>,
92}
93
94impl DashmapAccessor {
95    fn new(core: DashmapCore, root: String) -> Self {
96        let info = AccessorInfo::default();
97        info.set_scheme(DASHMAP_SCHEME);
98        info.set_name("dashmap");
99        info.set_root(&root);
100        info.set_native_capability(Capability {
101            read: true,
102
103            write: true,
104            write_can_empty: true,
105            write_with_cache_control: true,
106            write_with_content_type: true,
107            write_with_content_disposition: true,
108            write_with_content_encoding: true,
109
110            delete: true,
111            stat: true,
112            list: true,
113            shared: false,
114            ..Default::default()
115        });
116
117        Self {
118            core: Arc::new(core),
119            root,
120            info: Arc::new(info),
121        }
122    }
123}
124
125impl Access for DashmapAccessor {
126    type Reader = Buffer;
127    type Writer = DashmapWriter;
128    type Lister = oio::HierarchyLister<DashmapLister>;
129    type Deleter = oio::OneShotDeleter<DashmapDeleter>;
130
131    fn info(&self) -> Arc<AccessorInfo> {
132        self.info.clone()
133    }
134
135    async fn stat(&self, path: &str, _: OpStat) -> Result<RpStat> {
136        let p = build_abs_path(&self.root, path);
137
138        match self.core.get(&p)? {
139            Some(value) => {
140                let metadata = value.metadata;
141                Ok(RpStat::new(metadata))
142            }
143            None => {
144                if p.ends_with('/') {
145                    let has_children = self.core.cache.iter().any(|kv| kv.key().starts_with(&p));
146                    if has_children {
147                        return Ok(RpStat::new(Metadata::new(EntryMode::DIR)));
148                    }
149                }
150                Err(Error::new(ErrorKind::NotFound, "key not found in dashmap"))
151            }
152        }
153    }
154
155    async fn read(&self, path: &str, args: OpRead) -> Result<(RpRead, Self::Reader)> {
156        let p = build_abs_path(&self.root, path);
157
158        match self.core.get(&p)? {
159            Some(value) => {
160                let buffer = if args.range().is_full() {
161                    value.content
162                } else {
163                    let range = args.range();
164                    let start = range.offset() as usize;
165                    let end = match range.size() {
166                        Some(size) => (range.offset() + size) as usize,
167                        None => value.content.len(),
168                    };
169                    value.content.slice(start..end.min(value.content.len()))
170                };
171                Ok((RpRead::new(), buffer))
172            }
173            None => Err(Error::new(ErrorKind::NotFound, "key not found in dashmap")),
174        }
175    }
176
177    async fn write(&self, path: &str, args: OpWrite) -> Result<(RpWrite, Self::Writer)> {
178        let p = build_abs_path(&self.root, path);
179        Ok((
180            RpWrite::new(),
181            DashmapWriter::new(self.core.clone(), p, args),
182        ))
183    }
184
185    async fn delete(&self) -> Result<(RpDelete, Self::Deleter)> {
186        Ok((
187            RpDelete::default(),
188            oio::OneShotDeleter::new(DashmapDeleter::new(self.core.clone(), self.root.clone())),
189        ))
190    }
191
192    async fn list(&self, path: &str, args: OpList) -> Result<(RpList, Self::Lister)> {
193        let lister = DashmapLister::new(self.core.clone(), self.root.clone(), path.to_string());
194        let lister = oio::HierarchyLister::new(lister, path, args.recursive());
195        Ok((RpList::default(), lister))
196    }
197}
```
