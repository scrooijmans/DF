# 

opendal/services/sled/

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
20use std::str;
21
22use crate::Builder;
23use crate::Error;
24use crate::ErrorKind;
25use crate::Scheme;
26use crate::raw::adapters::kv;
27use crate::raw::*;
28use crate::services::SledConfig;
29use crate::*;
30
31// https://github.com/spacejam/sled/blob/69294e59c718289ab3cb6bd03ac3b9e1e072a1e7/src/db.rs#L5
32const DEFAULT_TREE_ID: &str = r#"__sled__default"#;
33
34/// Sled services support.
35#[doc = include_str!("docs.md")]
36#[derive(Default)]
37pub struct SledBuilder {
38    pub(super) config: SledConfig,
39}
40
41impl Debug for SledBuilder {
42    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
43        f.debug_struct("SledBuilder")
44            .field("config", &self.config)
45            .finish()
46    }
47}
48
49impl SledBuilder {
50    /// Set the path to the sled data directory. Will create if not exists.
51    pub fn datadir(mut self, path: &str) -> Self {
52        self.config.datadir = Some(path.into());
53        self
54    }
55
56    /// Set the root for sled.
57    pub fn root(mut self, root: &str) -> Self {
58        self.config.root = if root.is_empty() {
59            None
60        } else {
61            Some(root.to_string())
62        };
63
64        self
65    }
66
67    /// Set the tree for sled.
68    pub fn tree(mut self, tree: &str) -> Self {
69        self.config.tree = Some(tree.into());
70        self
71    }
72}
73
74impl Builder for SledBuilder {
75    type Config = SledConfig;
76
77    fn build(self) -> Result<impl Access> {
78        let datadir_path = self.config.datadir.ok_or_else(|| {
79            Error::new(ErrorKind::ConfigInvalid, "datadir is required but not set")
80                .with_context("service", Scheme::Sled)
81        })?;
82
83        let db = sled::open(&datadir_path).map_err(|e| {
84            Error::new(ErrorKind::ConfigInvalid, "open db")
85                .with_context("service", Scheme::Sled)
86                .with_context("datadir", datadir_path.clone())
87                .set_source(e)
88        })?;
89
90        // use "default" tree if not set
91        let tree_name = self
92            .config
93            .tree
94            .unwrap_or_else(|| DEFAULT_TREE_ID.to_string());
95
96        let tree = db.open_tree(&tree_name).map_err(|e| {
97            Error::new(ErrorKind::ConfigInvalid, "open tree")
98                .with_context("service", Scheme::Sled)
99                .with_context("datadir", datadir_path.clone())
100                .with_context("tree", tree_name.clone())
101                .set_source(e)
102        })?;
103
104        Ok(SledBackend::new(Adapter {
105            datadir: datadir_path,
106            tree,
107        })
108        .with_root(self.config.root.as_deref().unwrap_or("/")))
109    }
110}
111
112/// Backend for sled services.
113pub type SledBackend = kv::Backend<Adapter>;
114
115#[derive(Clone)]
116pub struct Adapter {
117    datadir: String,
118    tree: sled::Tree,
119}
120
121impl Debug for Adapter {
122    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
123        let mut ds = f.debug_struct("Adapter");
124        ds.field("path", &self.datadir);
125        ds.finish()
126    }
127}
128
129impl kv::Adapter for Adapter {
130    type Scanner = kv::Scanner;
131
132    fn info(&self) -> kv::Info {
133        kv::Info::new(
134            Scheme::Sled,
135            &self.datadir,
136            Capability {
137                read: true,
138                write: true,
139                list: true,
140                shared: false,
141                ..Default::default()
142            },
143        )
144    }
145
146    async fn get(&self, path: &str) -> Result<Option<Buffer>> {
147        Ok(self
148            .tree
149            .get(path)
150            .map_err(parse_error)?
151            .map(|v| Buffer::from(v.to_vec())))
152    }
153
154    async fn set(&self, path: &str, value: Buffer) -> Result<()> {
155        self.tree
156            .insert(path, value.to_vec())
157            .map_err(parse_error)?;
158        Ok(())
159    }
160
161    async fn delete(&self, path: &str) -> Result<()> {
162        self.tree.remove(path).map_err(parse_error)?;
163
164        Ok(())
165    }
166
167    async fn scan(&self, path: &str) -> Result<Self::Scanner> {
168        let it = self.tree.scan_prefix(path).keys();
169        let mut res = Vec::default();
170
171        for i in it {
172            let bs = i.map_err(parse_error)?.to_vec();
173            let v = String::from_utf8(bs).map_err(|err| {
174                Error::new(ErrorKind::Unexpected, "store key is not valid utf-8 string")
175                    .set_source(err)
176            })?;
177
178            res.push(v);
179        }
180
181        Ok(Box::new(kv::ScanStdIter::new(res.into_iter().map(Ok))))
182    }
183}
184
185fn parse_error(err: sled::Error) -> Error {
186    Error::new(ErrorKind::Unexpected, "error from sled").set_source(err)
187}
```
