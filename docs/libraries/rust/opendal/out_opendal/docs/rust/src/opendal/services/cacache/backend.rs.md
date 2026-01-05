# 

opendal/services/cacache/

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
18use crate::raw::*;
19use crate::services::CacacheConfig;
20use crate::*;
21use std::fmt::Debug;
22use std::sync::Arc;
23
24use super::CACACHE_SCHEME;
25use super::core::CacacheCore;
26use super::delete::CacacheDeleter;
27use super::writer::CacacheWriter;
28
29/// cacache service support.
30#[doc = include_str!("docs.md")]
31#[derive(Default)]
32pub struct CacacheBuilder {
33    pub(super) config: CacacheConfig,
34}
35
36impl CacacheBuilder {
37    /// Set the path to the cacache data directory. Will create if not exists.
38    pub fn datadir(mut self, path: &str) -> Self {
39        self.config.datadir = Some(path.into());
40        self
41    }
42}
43
44impl Builder for CacacheBuilder {
45    type Config = CacacheConfig;
46
47    fn build(self) -> Result<impl Access> {
48        let datadir_path = self.config.datadir.ok_or_else(|| {
49            Error::new(ErrorKind::ConfigInvalid, "datadir is required but not set")
50                .with_context("service", Scheme::Cacache)
51        })?;
52
53        let core = CacacheCore {
54            path: datadir_path.clone(),
55        };
56
57        let info = AccessorInfo::default();
58        info.set_scheme(CACACHE_SCHEME);
59        info.set_name(&datadir_path);
60        info.set_root("/");
61        info.set_native_capability(Capability {
62            read: true,
63            write: true,
64            delete: true,
65            stat: true,
66            rename: false,
67            list: false,
68            shared: false,
69            ..Default::default()
70        });
71
72        Ok(CacacheAccessor {
73            core: Arc::new(core),
74            info: Arc::new(info),
75        })
76    }
77}
78
79/// Backend for cacache services.
80#[derive(Debug, Clone)]
81pub struct CacacheAccessor {
82    core: Arc<CacacheCore>,
83    info: Arc<AccessorInfo>,
84}
85
86impl Access for CacacheAccessor {
87    type Reader = Buffer;
88    type Writer = CacacheWriter;
89    type Lister = ();
90    type Deleter = oio::OneShotDeleter<CacacheDeleter>;
91
92    fn info(&self) -> Arc<AccessorInfo> {
93        self.info.clone()
94    }
95
96    async fn stat(&self, path: &str, _: OpStat) -> Result<RpStat> {
97        let metadata = self.core.metadata(path).await?;
98
99        match metadata {
100            Some(meta) => {
101                let mut md = Metadata::new(EntryMode::FILE);
102                md.set_content_length(meta.size as u64);
103                // Convert u128 milliseconds to Timestamp
104                let millis = meta.time as i64;
105                if let Ok(dt) = Timestamp::from_millisecond(millis) {
106                    md.set_last_modified(dt);
107                }
108                Ok(RpStat::new(md))
109            }
110            None => Err(Error::new(ErrorKind::NotFound, "entry not found")),
111        }
112    }
113
114    async fn read(&self, path: &str, args: OpRead) -> Result<(RpRead, Self::Reader)> {
115        let data = self.core.get(path).await?;
116
117        match data {
118            Some(bytes) => {
119                let range = args.range();
120                let buffer = if range.is_full() {
121                    Buffer::from(bytes)
122                } else {
123                    let start = range.offset() as usize;
124                    let end = match range.size() {
125                        Some(size) => (range.offset() + size) as usize,
126                        None => bytes.len(),
127                    };
128                    Buffer::from(bytes.slice(start..end.min(bytes.len())))
129                };
130                Ok((RpRead::new(), buffer))
131            }
132            None => Err(Error::new(ErrorKind::NotFound, "entry not found")),
133        }
134    }
135
136    async fn write(&self, path: &str, _: OpWrite) -> Result<(RpWrite, Self::Writer)> {
137        Ok((
138            RpWrite::new(),
139            CacacheWriter::new(self.core.clone(), path.to_string()),
140        ))
141    }
142
143    async fn delete(&self) -> Result<(RpDelete, Self::Deleter)> {
144        Ok((
145            RpDelete::default(),
146            oio::OneShotDeleter::new(CacacheDeleter::new(self.core.clone())),
147        ))
148    }
149}
```
