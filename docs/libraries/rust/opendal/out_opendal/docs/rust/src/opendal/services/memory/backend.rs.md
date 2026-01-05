# 

opendal/services/memory/

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
19use std::sync::Arc;
20
21use super::MEMORY_SCHEME;
22use super::core::*;
23use super::delete::MemoryDeleter;
24use super::lister::MemoryLister;
25use super::writer::MemoryWriter;
26use crate::raw::oio;
27use crate::raw::*;
28use crate::services::MemoryConfig;
29use crate::*;
30
31/// In memory service support. (BTreeMap Based)
32#[doc = include_str!("docs.md")]
33#[derive(Default)]
34pub struct MemoryBuilder {
35    pub(super) config: MemoryConfig,
36}
37
38impl MemoryBuilder {
39    /// Set the root for BTreeMap.
40    pub fn root(mut self, path: &str) -> Self {
41        self.config.root = Some(path.into());
42        self
43    }
44}
45
46impl Builder for MemoryBuilder {
47    type Config = MemoryConfig;
48
49    fn build(self) -> Result<impl Access> {
50        let root = normalize_root(self.config.root.as_deref().unwrap_or("/"));
51
52        let core = MemoryCore::new();
53        Ok(MemoryAccessor::new(core).with_normalized_root(root))
54    }
55}
56
57/// MemoryAccessor implements Access trait directly
58#[derive(Debug, Clone)]
59pub struct MemoryAccessor {
60    core: Arc<MemoryCore>,
61    root: String,
62    info: Arc<AccessorInfo>,
63}
64
65impl MemoryAccessor {
66    fn new(core: MemoryCore) -> Self {
67        let info = AccessorInfo::default();
68        info.set_scheme(MEMORY_SCHEME);
69        info.set_name(&format!("{:p}", Arc::as_ptr(&core.data)));
70        info.set_root("/");
71        info.set_native_capability(Capability {
72            read: true,
73            write: true,
74            write_can_empty: true,
75            write_with_cache_control: true,
76            write_with_content_type: true,
77            write_with_content_disposition: true,
78            write_with_content_encoding: true,
79            delete: true,
80            stat: true,
81            list: true,
82            list_with_recursive: true,
83            shared: false,
84            ..Default::default()
85        });
86
87        Self {
88            core: Arc::new(core),
89            root: "/".to_string(),
90            info: Arc::new(info),
91        }
92    }
93
94    fn with_normalized_root(mut self, root: String) -> Self {
95        self.info.set_root(&root);
96        self.root = root;
97        self
98    }
99}
100
101impl Access for MemoryAccessor {
102    type Reader = Buffer;
103    type Writer = MemoryWriter;
104    type Lister = oio::HierarchyLister<MemoryLister>;
105    type Deleter = oio::OneShotDeleter<MemoryDeleter>;
106
107    fn info(&self) -> Arc<AccessorInfo> {
108        self.info.clone()
109    }
110
111    async fn stat(&self, path: &str, _: OpStat) -> Result<RpStat> {
112        let p = build_abs_path(&self.root, path);
113
114        if p == build_abs_path(&self.root, "") {
115            Ok(RpStat::new(Metadata::new(EntryMode::DIR)))
116        } else {
117            match self.core.get(&p)? {
118                Some(value) => Ok(RpStat::new(value.metadata)),
119                None => Err(Error::new(
120                    ErrorKind::NotFound,
121                    "memory doesn't have this path",
122                )),
123            }
124        }
125    }
126
127    async fn read(&self, path: &str, args: OpRead) -> Result<(RpRead, Self::Reader)> {
128        let p = build_abs_path(&self.root, path);
129
130        let value = match self.core.get(&p)? {
131            Some(value) => value,
132            None => {
133                return Err(Error::new(
134                    ErrorKind::NotFound,
135                    "memory doesn't have this path",
136                ));
137            }
138        };
139
140        Ok((
141            RpRead::new(),
142            value.content.slice(args.range().to_range_as_usize()),
143        ))
144    }
145
146    async fn write(&self, path: &str, args: OpWrite) -> Result<(RpWrite, Self::Writer)> {
147        let p = build_abs_path(&self.root, path);
148        Ok((
149            RpWrite::new(),
150            MemoryWriter::new(self.core.clone(), p, args),
151        ))
152    }
153
154    async fn delete(&self) -> Result<(RpDelete, Self::Deleter)> {
155        Ok((
156            RpDelete::default(),
157            oio::OneShotDeleter::new(MemoryDeleter::new(self.core.clone(), self.root.clone())),
158        ))
159    }
160
161    async fn list(&self, path: &str, args: OpList) -> Result<(RpList, Self::Lister)> {
162        let p = build_abs_path(&self.root, path);
163        let keys = self.core.scan(&p)?;
164        let lister = MemoryLister::new(&self.root, keys);
165        let lister = oio::HierarchyLister::new(lister, path, args.recursive());
166
167        Ok((RpList::default(), lister))
168    }
169}
```
