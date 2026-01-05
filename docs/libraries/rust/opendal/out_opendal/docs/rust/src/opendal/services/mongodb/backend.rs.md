# 

opendal/services/mongodb/

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
22use super::config::MongodbConfig;
23use super::core::*;
24use super::deleter::MongodbDeleter;
25use super::writer::MongodbWriter;
26use crate::raw::*;
27use crate::*;
28
29#[doc = include_str!("docs.md")]
30#[derive(Default)]
31pub struct MongodbBuilder {
32    pub(super) config: MongodbConfig,
33}
34
35impl MongodbBuilder {
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
62    /// Set the working directory, all operations will be performed under it.
63    ///
64    /// default: "/"
65    pub fn root(mut self, root: &str) -> Self {
66        self.config.root = if root.is_empty() {
67            None
68        } else {
69            Some(root.to_string())
70        };
71
72        self
73    }
74
75    /// Set the database name of the MongoDB service to read/write.
76    pub fn database(mut self, database: &str) -> Self {
77        if !database.is_empty() {
78            self.config.database = Some(database.to_string());
79        }
80        self
81    }
82
83    /// Set the collection name of the MongoDB service to read/write.
84    pub fn collection(mut self, collection: &str) -> Self {
85        if !collection.is_empty() {
86            self.config.collection = Some(collection.to_string());
87        }
88        self
89    }
90
91    /// Set the key field name of the MongoDB service to read/write.
92    ///
93    /// Default to `key` if not specified.
94    pub fn key_field(mut self, key_field: &str) -> Self {
95        if !key_field.is_empty() {
96            self.config.key_field = Some(key_field.to_string());
97        }
98        self
99    }
100
101    /// Set the value field name of the MongoDB service to read/write.
102    ///
103    /// Default to `value` if not specified.
104    pub fn value_field(mut self, value_field: &str) -> Self {
105        if !value_field.is_empty() {
106            self.config.value_field = Some(value_field.to_string());
107        }
108        self
109    }
110}
111
112impl Builder for MongodbBuilder {
113    type Config = MongodbConfig;
114
115    fn build(self) -> Result<impl Access> {
116        let conn = match &self.config.connection_string.clone() {
117            Some(v) => v.clone(),
118            None => {
119                return Err(
120                    Error::new(ErrorKind::ConfigInvalid, "connection_string is required")
121                        .with_context("service", Scheme::Mongodb),
122                );
123            }
124        };
125        let database = match &self.config.database.clone() {
126            Some(v) => v.clone(),
127            None => {
128                return Err(Error::new(ErrorKind::ConfigInvalid, "database is required")
129                    .with_context("service", Scheme::Mongodb));
130            }
131        };
132        let collection = match &self.config.collection.clone() {
133            Some(v) => v.clone(),
134            None => {
135                return Err(
136                    Error::new(ErrorKind::ConfigInvalid, "collection is required")
137                        .with_context("service", Scheme::Mongodb),
138                );
139            }
140        };
141        let key_field = match &self.config.key_field.clone() {
142            Some(v) => v.clone(),
143            None => "key".to_string(),
144        };
145        let value_field = match &self.config.value_field.clone() {
146            Some(v) => v.clone(),
147            None => "value".to_string(),
148        };
149        let root = normalize_root(
150            self.config
151                .root
152                .clone()
153                .unwrap_or_else(|| "/".to_string())
154                .as_str(),
155        );
156        Ok(MongodbBackend::new(MongodbCore {
157            connection_string: conn,
158            database,
159            collection,
160            collection_instance: OnceCell::new(),
161            key_field,
162            value_field,
163        })
164        .with_normalized_root(root))
165    }
166}
167
168/// Backend for Mongodb services.
169#[derive(Clone, Debug)]
170pub struct MongodbBackend {
171    core: Arc<MongodbCore>,
172    root: String,
173    info: Arc<AccessorInfo>,
174}
175
176impl MongodbBackend {
177    pub fn new(core: MongodbCore) -> Self {
178        let info = AccessorInfo::default();
179        info.set_scheme(Scheme::Mongodb.into_static());
180        info.set_name(&format!("{}/{}", core.database, core.collection));
181        info.set_root("/");
182        info.set_native_capability(Capability {
183            read: true,
184            stat: true,
185            write: true,
186            write_can_empty: true,
187            delete: true,
188            shared: true,
189            ..Default::default()
190        });
191
192        Self {
193            core: Arc::new(core),
194            root: "/".to_string(),
195            info: Arc::new(info),
196        }
197    }
198
199    fn with_normalized_root(mut self, root: String) -> Self {
200        self.info.set_root(&root);
201        self.root = root;
202        self
203    }
204}
205
206impl Access for MongodbBackend {
207    type Reader = Buffer;
208    type Writer = MongodbWriter;
209    type Lister = ();
210    type Deleter = oio::OneShotDeleter<MongodbDeleter>;
211
212    fn info(&self) -> Arc<AccessorInfo> {
213        self.info.clone()
214    }
215
216    async fn stat(&self, path: &str, _: OpStat) -> Result<RpStat> {
217        let p = build_abs_path(&self.root, path);
218
219        if p == build_abs_path(&self.root, "") {
220            Ok(RpStat::new(Metadata::new(EntryMode::DIR)))
221        } else {
222            let bs = self.core.get(&p).await?;
223            match bs {
224                Some(bs) => Ok(RpStat::new(
225                    Metadata::new(EntryMode::FILE).with_content_length(bs.len() as u64),
226                )),
227                None => Err(Error::new(ErrorKind::NotFound, "kv not found in mongodb")),
228            }
229        }
230    }
231
232    async fn read(&self, path: &str, args: OpRead) -> Result<(RpRead, Self::Reader)> {
233        let p = build_abs_path(&self.root, path);
234        let bs = match self.core.get(&p).await? {
235            Some(bs) => bs,
236            None => {
237                return Err(Error::new(ErrorKind::NotFound, "kv not found in mongodb"));
238            }
239        };
240        Ok((RpRead::new(), bs.slice(args.range().to_range_as_usize())))
241    }
242
243    async fn write(&self, path: &str, _: OpWrite) -> Result<(RpWrite, Self::Writer)> {
244        let p = build_abs_path(&self.root, path);
245        Ok((RpWrite::new(), MongodbWriter::new(self.core.clone(), p)))
246    }
247
248    async fn delete(&self) -> Result<(RpDelete, Self::Deleter)> {
249        Ok((
250            RpDelete::default(),
251            oio::OneShotDeleter::new(MongodbDeleter::new(self.core.clone(), self.root.clone())),
252        ))
253    }
254
255    async fn list(&self, path: &str, _: OpList) -> Result<(RpList, Self::Lister)> {
256        let _ = build_abs_path(&self.root, path);
257        Ok((RpList::default(), ()))
258    }
259}
```
