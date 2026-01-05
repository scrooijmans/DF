# 

opendal/services/rocksdb/

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
22use rocksdb::DB;
23
24use crate::Result;
25use crate::raw::adapters::kv;
26use crate::raw::*;
27use crate::services::RocksdbConfig;
28use crate::*;
29
30/// RocksDB service support.
31#[doc = include_str!("docs.md")]
32#[derive(Clone, Default)]
33pub struct RocksdbBuilder {
34    pub(super) config: RocksdbConfig,
35}
36
37impl RocksdbBuilder {
38    /// Set the path to the rocksdb data directory. Will create if not exists.
39    pub fn datadir(mut self, path: &str) -> Self {
40        self.config.datadir = Some(path.into());
41        self
42    }
43
44    /// set the working directory, all operations will be performed under it.
45    ///
46    /// default: "/"
47    pub fn root(mut self, root: &str) -> Self {
48        self.config.root = if root.is_empty() {
49            None
50        } else {
51            Some(root.to_string())
52        };
53
54        self
55    }
56}
57
58impl Builder for RocksdbBuilder {
59    type Config = RocksdbConfig;
60
61    fn build(self) -> Result<impl Access> {
62        let path = self.config.datadir.ok_or_else(|| {
63            Error::new(ErrorKind::ConfigInvalid, "datadir is required but not set")
64                .with_context("service", Scheme::Rocksdb)
65        })?;
66        let db = DB::open_default(&path).map_err(|e| {
67            Error::new(ErrorKind::ConfigInvalid, "open default transaction db")
68                .with_context("service", Scheme::Rocksdb)
69                .with_context("datadir", path)
70                .set_source(e)
71        })?;
72
73        let root = normalize_root(
74            self.config
75                .root
76                .clone()
77                .unwrap_or_else(|| "/".to_string())
78                .as_str(),
79        );
80
81        Ok(RocksdbBackend::new(Adapter { db: Arc::new(db) }).with_normalized_root(root))
82    }
83}
84
85/// Backend for rocksdb services.
86pub type RocksdbBackend = kv::Backend<Adapter>;
87
88#[derive(Clone)]
89pub struct Adapter {
90    db: Arc<DB>,
91}
92
93impl Debug for Adapter {
94    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
95        let mut ds = f.debug_struct("Adapter");
96        ds.field("path", &self.db.path());
97        ds.finish()
98    }
99}
100
101impl kv::Adapter for Adapter {
102    type Scanner = kv::Scanner;
103
104    fn info(&self) -> kv::Info {
105        kv::Info::new(
106            Scheme::Rocksdb,
107            &self.db.path().to_string_lossy(),
108            Capability {
109                read: true,
110                write: true,
111                list: true,
112                shared: false,
113                ..Default::default()
114            },
115        )
116    }
117
118    async fn get(&self, path: &str) -> Result<Option<Buffer>> {
119        let result = self.db.get(path).map_err(parse_rocksdb_error)?;
120        Ok(result.map(Buffer::from))
121    }
122
123    async fn set(&self, path: &str, value: Buffer) -> Result<()> {
124        self.db
125            .put(path, value.to_vec())
126            .map_err(parse_rocksdb_error)
127    }
128
129    async fn delete(&self, path: &str) -> Result<()> {
130        self.db.delete(path).map_err(parse_rocksdb_error)
131    }
132
133    async fn scan(&self, path: &str) -> Result<Self::Scanner> {
134        let it = self.db.prefix_iterator(path).map(|r| r.map(|(k, _)| k));
135        let mut res = Vec::default();
136
137        for key in it {
138            let key = key.map_err(parse_rocksdb_error)?;
139            let key = String::from_utf8_lossy(&key);
140            if !key.starts_with(path) {
141                break;
142            }
143            res.push(key.to_string());
144        }
145
146        Ok(Box::new(kv::ScanStdIter::new(res.into_iter().map(Ok))))
147    }
148}
149
150fn parse_rocksdb_error(e: rocksdb::Error) -> Error {
151    Error::new(ErrorKind::Unexpected, "got rocksdb error").set_source(e)
152}
```
