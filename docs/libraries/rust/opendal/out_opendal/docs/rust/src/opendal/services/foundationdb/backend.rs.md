# 

opendal/services/foundationdb/

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
21use foundationdb::Database;
22
23use super::config::FoundationdbConfig;
24use super::core::*;
25use super::deleter::FoundationdbDeleter;
26use super::writer::FoundationdbWriter;
27use crate::raw::*;
28use crate::*;
29
30#[doc = include_str!("docs.md")]
31#[derive(Default)]
32pub struct FoundationdbBuilder {
33    pub(super) config: FoundationdbConfig,
34}
35
36impl FoundationdbBuilder {
37    /// Set the root for Foundationdb.
38    pub fn root(mut self, path: &str) -> Self {
39        self.config.root = Some(path.into());
40        self
41    }
42
43    /// Set the config path for Foundationdb. If not set, will fallback to use default
44    pub fn config_path(mut self, path: &str) -> Self {
45        self.config.config_path = Some(path.into());
46        self
47    }
48}
49
50impl Builder for FoundationdbBuilder {
51    type Config = FoundationdbConfig;
52
53    fn build(self) -> Result<impl Access> {
54        let _network = Arc::new(unsafe { foundationdb::boot() });
55        let db;
56        if let Some(cfg_path) = &self.config.config_path {
57            db = Database::from_path(cfg_path).map_err(|e| {
58                Error::new(ErrorKind::ConfigInvalid, "open foundation db")
59                    .with_context("service", Scheme::Foundationdb)
60                    .set_source(e)
61            })?;
62        } else {
63            db = Database::default().map_err(|e| {
64                Error::new(ErrorKind::ConfigInvalid, "open foundation db")
65                    .with_context("service", Scheme::Foundationdb)
66                    .set_source(e)
67            })?
68        }
69
70        let db = Arc::new(db);
71
72        let root = normalize_root(
73            self.config
74                .root
75                .clone()
76                .unwrap_or_else(|| "/".to_string())
77                .as_str(),
78        );
79
80        Ok(FoundationdbBackend::new(FoundationdbCore { db, _network }).with_normalized_root(root))
81    }
82}
83
84/// Backend for Foundationdb services.
85#[derive(Clone, Debug)]
86pub struct FoundationdbBackend {
87    core: Arc<FoundationdbCore>,
88    root: String,
89    info: Arc<AccessorInfo>,
90}
91
92impl FoundationdbBackend {
93    pub fn new(core: FoundationdbCore) -> Self {
94        let info = AccessorInfo::default();
95        info.set_scheme(Scheme::Foundationdb.into_static());
96        info.set_name("foundationdb");
97        info.set_root("/");
98        info.set_native_capability(Capability {
99            read: true,
100            stat: true,
101            write: true,
102            write_can_empty: true,
103            delete: true,
104            shared: true,
105            ..Default::default()
106        });
107
108        Self {
109            core: Arc::new(core),
110            root: "/".to_string(),
111            info: Arc::new(info),
112        }
113    }
114
115    fn with_normalized_root(mut self, root: String) -> Self {
116        self.info.set_root(&root);
117        self.root = root;
118        self
119    }
120}
121
122impl Access for FoundationdbBackend {
123    type Reader = Buffer;
124    type Writer = FoundationdbWriter;
125    type Lister = ();
126    type Deleter = oio::OneShotDeleter<FoundationdbDeleter>;
127
128    fn info(&self) -> Arc<AccessorInfo> {
129        self.info.clone()
130    }
131
132    async fn stat(&self, path: &str, _: OpStat) -> Result<RpStat> {
133        let p = build_abs_path(&self.root, path);
134
135        if p == build_abs_path(&self.root, "") {
136            Ok(RpStat::new(Metadata::new(EntryMode::DIR)))
137        } else {
138            let bs = self.core.get(&p).await?;
139            match bs {
140                Some(bs) => Ok(RpStat::new(
141                    Metadata::new(EntryMode::FILE).with_content_length(bs.len() as u64),
142                )),
143                None => Err(Error::new(
144                    ErrorKind::NotFound,
145                    "kv not found in foundationdb",
146                )),
147            }
148        }
149    }
150
151    async fn read(&self, path: &str, args: OpRead) -> Result<(RpRead, Self::Reader)> {
152        let p = build_abs_path(&self.root, path);
153        let bs = match self.core.get(&p).await? {
154            Some(bs) => bs,
155            None => {
156                return Err(Error::new(
157                    ErrorKind::NotFound,
158                    "kv not found in foundationdb",
159                ));
160            }
161        };
162        Ok((RpRead::new(), bs.slice(args.range().to_range_as_usize())))
163    }
164
165    async fn write(&self, path: &str, _: OpWrite) -> Result<(RpWrite, Self::Writer)> {
166        let p = build_abs_path(&self.root, path);
167        Ok((
168            RpWrite::new(),
169            FoundationdbWriter::new(self.core.clone(), p),
170        ))
171    }
172
173    async fn delete(&self) -> Result<(RpDelete, Self::Deleter)> {
174        Ok((
175            RpDelete::default(),
176            oio::OneShotDeleter::new(FoundationdbDeleter::new(
177                self.core.clone(),
178                self.root.clone(),
179            )),
180        ))
181    }
182
183    async fn list(&self, path: &str, _: OpList) -> Result<(RpList, Self::Lister)> {
184        let _ = build_abs_path(&self.root, path);
185        Ok((RpList::default(), ()))
186    }
187}
```
