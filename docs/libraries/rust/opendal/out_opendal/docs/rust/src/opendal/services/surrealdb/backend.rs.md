# 

opendal/services/surrealdb/

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
22use tokio::sync::OnceCell;
23
24use super::config::SurrealdbConfig;
25use super::core::*;
26use super::deleter::SurrealdbDeleter;
27use super::writer::SurrealdbWriter;
28use crate::raw::*;
29use crate::*;
30
31#[doc = include_str!("docs.md")]
32#[derive(Default)]
33pub struct SurrealdbBuilder {
34    pub(super) config: SurrealdbConfig,
35}
36
37impl Debug for SurrealdbBuilder {
38    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
39        f.debug_struct("SurrealdbBuilder")
40            .field("config", &self.config)
41            .finish()
42    }
43}
44
45impl SurrealdbBuilder {
46    /// Set the connection_string of the surrealdb service.
47    ///
48    /// This connection string is used to connect to the surrealdb service. There are url based formats:
49    ///
50    /// ## Url
51    ///
52    /// - `ws://ip:port`
53    /// - `wss://ip:port`
54    /// - `http://ip:port`
55    /// - `https://ip:port`
56    pub fn connection_string(mut self, connection_string: &str) -> Self {
57        if !connection_string.is_empty() {
58            self.config.connection_string = Some(connection_string.to_string());
59        }
60        self
61    }
62
63    /// set the working directory, all operations will be performed under it.
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
76    /// Set the table name of the surrealdb service for read/write.
77    pub fn table(mut self, table: &str) -> Self {
78        if !table.is_empty() {
79            self.config.table = Some(table.to_string());
80        }
81        self
82    }
83
84    /// Set the username of the surrealdb service for signin.
85    pub fn username(mut self, username: &str) -> Self {
86        if !username.is_empty() {
87            self.config.username = Some(username.to_string());
88        }
89        self
90    }
91
92    /// Set the password of the surrealdb service for signin.
93    pub fn password(mut self, password: &str) -> Self {
94        if !password.is_empty() {
95            self.config.password = Some(password.to_string());
96        }
97        self
98    }
99
100    /// Set the namespace of the surrealdb service for read/write.
101    pub fn namespace(mut self, namespace: &str) -> Self {
102        if !namespace.is_empty() {
103            self.config.namespace = Some(namespace.to_string());
104        }
105        self
106    }
107
108    /// Set the database of the surrealdb service for read/write.
109    pub fn database(mut self, database: &str) -> Self {
110        if !database.is_empty() {
111            self.config.database = Some(database.to_string());
112        }
113        self
114    }
115
116    /// Set the key field name of the surrealdb service for read/write.
117    ///
118    /// Default to `key` if not specified.
119    pub fn key_field(mut self, key_field: &str) -> Self {
120        if !key_field.is_empty() {
121            self.config.key_field = Some(key_field.to_string());
122        }
123        self
124    }
125
126    /// Set the value field name of the surrealdb service for read/write.
127    ///
128    /// Default to `value` if not specified.
129    pub fn value_field(mut self, value_field: &str) -> Self {
130        if !value_field.is_empty() {
131            self.config.value_field = Some(value_field.to_string());
132        }
133        self
134    }
135}
136
137impl Builder for SurrealdbBuilder {
138    type Config = SurrealdbConfig;
139
140    fn build(self) -> Result<impl Access> {
141        let connection_string = match self.config.connection_string.clone() {
142            Some(v) => v,
143            None => {
144                return Err(
145                    Error::new(ErrorKind::ConfigInvalid, "connection_string is empty")
146                        .with_context("service", Scheme::Surrealdb),
147                );
148            }
149        };
150
151        let namespace = match self.config.namespace.clone() {
152            Some(v) => v,
153            None => {
154                return Err(Error::new(ErrorKind::ConfigInvalid, "namespace is empty")
155                    .with_context("service", Scheme::Surrealdb));
156            }
157        };
158        let database = match self.config.database.clone() {
159            Some(v) => v,
160            None => {
161                return Err(Error::new(ErrorKind::ConfigInvalid, "database is empty")
162                    .with_context("service", Scheme::Surrealdb));
163            }
164        };
165        let table = match self.config.table.clone() {
166            Some(v) => v,
167            None => {
168                return Err(Error::new(ErrorKind::ConfigInvalid, "table is empty")
169                    .with_context("service", Scheme::Surrealdb));
170            }
171        };
172
173        let username = self.config.username.clone().unwrap_or_default();
174        let password = self.config.password.clone().unwrap_or_default();
175        let key_field = self
176            .config
177            .key_field
178            .clone()
179            .unwrap_or_else(|| "key".to_string());
180        let value_field = self
181            .config
182            .value_field
183            .clone()
184            .unwrap_or_else(|| "value".to_string());
185        let root = normalize_root(
186            self.config
187                .root
188                .clone()
189                .unwrap_or_else(|| "/".to_string())
190                .as_str(),
191        );
192
193        Ok(SurrealdbBackend::new(SurrealdbCore {
194            db: OnceCell::new(),
195            connection_string,
196            username,
197            password,
198            namespace,
199            database,
200            table,
201            key_field,
202            value_field,
203        })
204        .with_normalized_root(root))
205    }
206}
207
208/// Backend for Surrealdb service
209#[derive(Clone, Debug)]
210pub struct SurrealdbBackend {
211    core: Arc<SurrealdbCore>,
212    root: String,
213    info: Arc<AccessorInfo>,
214}
215
216impl SurrealdbBackend {
217    pub fn new(core: SurrealdbCore) -> Self {
218        let info = AccessorInfo::default();
219        info.set_scheme(Scheme::Surrealdb.into_static());
220        info.set_name(&core.table);
221        info.set_root("/");
222        info.set_native_capability(Capability {
223            read: true,
224            stat: true,
225            write: true,
226            write_can_empty: true,
227            delete: true,
228            shared: true,
229            ..Default::default()
230        });
231
232        Self {
233            core: Arc::new(core),
234            root: "/".to_string(),
235            info: Arc::new(info),
236        }
237    }
238
239    fn with_normalized_root(mut self, root: String) -> Self {
240        self.info.set_root(&root);
241        self.root = root;
242        self
243    }
244}
245
246impl Access for SurrealdbBackend {
247    type Reader = Buffer;
248    type Writer = SurrealdbWriter;
249    type Lister = ();
250    type Deleter = oio::OneShotDeleter<SurrealdbDeleter>;
251
252    fn info(&self) -> Arc<AccessorInfo> {
253        self.info.clone()
254    }
255
256    async fn stat(&self, path: &str, _: OpStat) -> Result<RpStat> {
257        let p = build_abs_path(&self.root, path);
258
259        if p == build_abs_path(&self.root, "") {
260            Ok(RpStat::new(Metadata::new(EntryMode::DIR)))
261        } else {
262            let bs = self.core.get(&p).await?;
263            match bs {
264                Some(bs) => Ok(RpStat::new(
265                    Metadata::new(EntryMode::FILE).with_content_length(bs.len() as u64),
266                )),
267                None => Err(Error::new(ErrorKind::NotFound, "kv not found in surrealdb")),
268            }
269        }
270    }
271
272    async fn read(&self, path: &str, args: OpRead) -> Result<(RpRead, Self::Reader)> {
273        let p = build_abs_path(&self.root, path);
274        let bs = match self.core.get(&p).await? {
275            Some(bs) => bs,
276            None => {
277                return Err(Error::new(ErrorKind::NotFound, "kv not found in surrealdb"));
278            }
279        };
280        Ok((RpRead::new(), bs.slice(args.range().to_range_as_usize())))
281    }
282
283    async fn write(&self, path: &str, _: OpWrite) -> Result<(RpWrite, Self::Writer)> {
284        let p = build_abs_path(&self.root, path);
285        Ok((RpWrite::new(), SurrealdbWriter::new(self.core.clone(), p)))
286    }
287
288    async fn delete(&self) -> Result<(RpDelete, Self::Deleter)> {
289        Ok((
290            RpDelete::default(),
291            oio::OneShotDeleter::new(SurrealdbDeleter::new(self.core.clone(), self.root.clone())),
292        ))
293    }
294
295    async fn list(&self, path: &str, _: OpList) -> Result<(RpList, Self::Lister)> {
296        let _ = build_abs_path(&self.root, path);
297        Ok((RpList::default(), ()))
298    }
299}
```
