# 

opendal/services/gridfs/

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
19
20use tokio::sync::OnceCell;
21
22use super::config::GridfsConfig;
23use super::core::*;
24use super::deleter::GridfsDeleter;
25use super::writer::GridfsWriter;
26use crate::raw::*;
27use crate::*;
28
29#[doc = include_str!("docs.md")]
30#[derive(Default)]
31pub struct GridfsBuilder {
32    pub(super) config: GridfsConfig,
33}
34
35impl GridfsBuilder {
36    /// Set the connection_string of the MongoDB service.
37    ///
38    /// This connection string is used to connect to the MongoDB service. It typically follows the format:
39    ///
40    /// ## Format
41    ///
42    /// `mongodb://[username:password@]host1[:port1][,...hostN[:portN]][/[defaultauthdb][?options]]`
43    ///
44    /// Examples:
45    ///
46    /// - Connecting to a local MongoDB instance: `mongodb://localhost:27017`
47    /// - Using authentication: `mongodb://myUser:myPassword@localhost:27017/myAuthDB`
48    /// - Specifying authentication mechanism: `mongodb://myUser:myPassword@localhost:27017/myAuthDB?authMechanism=SCRAM-SHA-256`
49    ///
50    /// ## Options
51    ///
52    /// - `authMechanism`: Specifies the authentication method to use. Examples include `SCRAM-SHA-1`, `SCRAM-SHA-256`, and `MONGODB-AWS`.
53    /// - ... (any other options you wish to highlight)
54    ///
55    /// For more information, please refer to [MongoDB Connection String URI Format](https://docs.mongodb.com/manual/reference/connection-string/).
56    pub fn connection_string(mut self, v: &str) -> Self {
57        if !v.is_empty() {
58            self.config.connection_string = Some(v.to_string());
59        }
60        self
61    }
62
63    /// Set the working directory, all operations will be performed under it.
64    ///
65    /// default: "/"
66    pub fn root(mut self, root: &str) -> Self {
67        self.config.root = if root.is_empty() {
68            None
69        } else {
70            Some(root.to_string())
71        };
72
73        self
74    }
75
76    /// Set the database name of the MongoDB GridFs service to read/write.
77    pub fn database(mut self, database: &str) -> Self {
78        if !database.is_empty() {
79            self.config.database = Some(database.to_string());
80        }
81        self
82    }
83
84    /// Set the bucket name of the MongoDB GridFs service to read/write.
85    ///
86    /// Default to `fs` if not specified.
87    pub fn bucket(mut self, bucket: &str) -> Self {
88        if !bucket.is_empty() {
89            self.config.bucket = Some(bucket.to_string());
90        }
91        self
92    }
93
94    /// Set the chunk size of the MongoDB GridFs service used to break the user file into chunks.
95    ///
96    /// Default to `255 KiB` if not specified.
97    pub fn chunk_size(mut self, chunk_size: u32) -> Self {
98        if chunk_size > 0 {
99            self.config.chunk_size = Some(chunk_size);
100        }
101        self
102    }
103}
104
105impl Builder for GridfsBuilder {
106    type Config = GridfsConfig;
107
108    fn build(self) -> Result<impl Access> {
109        let conn = match &self.config.connection_string.clone() {
110            Some(v) => v.clone(),
111            None => {
112                return Err(
113                    Error::new(ErrorKind::ConfigInvalid, "connection_string is required")
114                        .with_context("service", Scheme::Gridfs),
115                );
116            }
117        };
118        let database = match &self.config.database.clone() {
119            Some(v) => v.clone(),
120            None => {
121                return Err(Error::new(ErrorKind::ConfigInvalid, "database is required")
122                    .with_context("service", Scheme::Gridfs));
123            }
124        };
125        let bucket = match &self.config.bucket.clone() {
126            Some(v) => v.clone(),
127            None => "fs".to_string(),
128        };
129        let chunk_size = self.config.chunk_size.unwrap_or(255);
130
131        let root = normalize_root(
132            self.config
133                .root
134                .clone()
135                .unwrap_or_else(|| "/".to_string())
136                .as_str(),
137        );
138
139        Ok(GridfsBackend::new(GridfsCore {
140            connection_string: conn,
141            database,
142            bucket,
143            chunk_size,
144            bucket_instance: OnceCell::new(),
145        })
146        .with_normalized_root(root))
147    }
148}
149
150/// Backend for Gridfs services.
151#[derive(Clone, Debug)]
152pub struct GridfsBackend {
153    core: Arc<GridfsCore>,
154    root: String,
155    info: Arc<AccessorInfo>,
156}
157
158impl GridfsBackend {
159    pub fn new(core: GridfsCore) -> Self {
160        let info = AccessorInfo::default();
161        info.set_scheme(Scheme::Gridfs.into_static());
162        info.set_name(&format!("{}/{}", core.database, core.bucket));
163        info.set_root("/");
164        info.set_native_capability(Capability {
165            read: true,
166            stat: true,
167            write: true,
168            write_can_empty: true,
169            delete: true,
170            shared: true,
171            ..Default::default()
172        });
173
174        Self {
175            core: Arc::new(core),
176            root: "/".to_string(),
177            info: Arc::new(info),
178        }
179    }
180
181    fn with_normalized_root(mut self, root: String) -> Self {
182        self.info.set_root(&root);
183        self.root = root;
184        self
185    }
186}
187
188impl Access for GridfsBackend {
189    type Reader = Buffer;
190    type Writer = GridfsWriter;
191    type Lister = ();
192    type Deleter = oio::OneShotDeleter<GridfsDeleter>;
193
194    fn info(&self) -> Arc<AccessorInfo> {
195        self.info.clone()
196    }
197
198    async fn stat(&self, path: &str, _: OpStat) -> Result<RpStat> {
199        let p = build_abs_path(&self.root, path);
200
201        if p == build_abs_path(&self.root, "") {
202            Ok(RpStat::new(Metadata::new(EntryMode::DIR)))
203        } else {
204            let bs = self.core.get(&p).await?;
205            match bs {
206                Some(bs) => Ok(RpStat::new(
207                    Metadata::new(EntryMode::FILE).with_content_length(bs.len() as u64),
208                )),
209                None => Err(Error::new(ErrorKind::NotFound, "kv not found in gridfs")),
210            }
211        }
212    }
213
214    async fn read(&self, path: &str, args: OpRead) -> Result<(RpRead, Self::Reader)> {
215        let p = build_abs_path(&self.root, path);
216        let bs = match self.core.get(&p).await? {
217            Some(bs) => bs,
218            None => {
219                return Err(Error::new(ErrorKind::NotFound, "kv not found in gridfs"));
220            }
221        };
222        Ok((RpRead::new(), bs.slice(args.range().to_range_as_usize())))
223    }
224
225    async fn write(&self, path: &str, _: OpWrite) -> Result<(RpWrite, Self::Writer)> {
226        let p = build_abs_path(&self.root, path);
227        Ok((RpWrite::new(), GridfsWriter::new(self.core.clone(), p)))
228    }
229
230    async fn delete(&self) -> Result<(RpDelete, Self::Deleter)> {
231        Ok((
232            RpDelete::default(),
233            oio::OneShotDeleter::new(GridfsDeleter::new(self.core.clone(), self.root.clone())),
234        ))
235    }
236
237    async fn list(&self, path: &str, _: OpList) -> Result<(RpList, Self::Lister)> {
238        let _ = build_abs_path(&self.root, path);
239        Ok((RpList::default(), ()))
240    }
241}
```
