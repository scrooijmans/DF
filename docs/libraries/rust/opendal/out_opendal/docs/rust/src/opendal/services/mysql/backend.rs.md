# 

opendal/services/mysql/

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
21use sqlx::mysql::MySqlConnectOptions;
22use tokio::sync::OnceCell;
23
24use super::config::MysqlConfig;
25use super::core::*;
26use super::deleter::MysqlDeleter;
27use super::writer::MysqlWriter;
28use crate::raw::oio;
29use crate::raw::*;
30use crate::*;
31
32#[doc = include_str!("docs.md")]
33#[derive(Default)]
34pub struct MysqlBuilder {
35    pub(super) config: MysqlConfig,
36}
37
38impl Debug for MysqlBuilder {
39    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
40        let mut d = f.debug_struct("MysqlBuilder");
41
42        d.field("config", &self.config).finish()
43    }
44}
45
46impl MysqlBuilder {
47    /// Set the connection_string of the mysql service.
48    ///
49    /// This connection string is used to connect to the mysql service. There are url based formats:
50    ///
51    /// ## Url
52    ///
53    /// This format resembles the url format of the mysql client. The format is: `[scheme://][user[:[password]]@]host[:port][/schema][?attribute1=value1&attribute2=value2...`
54    ///
55    /// - `mysql://user@localhost`
56    /// - `mysql://user:password@localhost`
57    /// - `mysql://user:password@localhost:3306`
58    /// - `mysql://user:password@localhost:3306/db`
59    ///
60    /// For more information, please refer to <https://docs.rs/sqlx/latest/sqlx/mysql/struct.MySqlConnectOptions.html>.
61    pub fn connection_string(mut self, v: &str) -> Self {
62        if !v.is_empty() {
63            self.config.connection_string = Some(v.to_string());
64        }
65        self
66    }
67
68    /// set the working directory, all operations will be performed under it.
69    ///
70    /// default: "/"
71    pub fn root(mut self, root: &str) -> Self {
72        self.config.root = if root.is_empty() {
73            None
74        } else {
75            Some(root.to_string())
76        };
77
78        self
79    }
80
81    /// Set the table name of the mysql service to read/write.
82    pub fn table(mut self, table: &str) -> Self {
83        if !table.is_empty() {
84            self.config.table = Some(table.to_string());
85        }
86        self
87    }
88
89    /// Set the key field name of the mysql service to read/write.
90    ///
91    /// Default to `key` if not specified.
92    pub fn key_field(mut self, key_field: &str) -> Self {
93        if !key_field.is_empty() {
94            self.config.key_field = Some(key_field.to_string());
95        }
96        self
97    }
98
99    /// Set the value field name of the mysql service to read/write.
100    ///
101    /// Default to `value` if not specified.
102    pub fn value_field(mut self, value_field: &str) -> Self {
103        if !value_field.is_empty() {
104            self.config.value_field = Some(value_field.to_string());
105        }
106        self
107    }
108}
109
110impl Builder for MysqlBuilder {
111    type Config = MysqlConfig;
112
113    fn build(self) -> Result<impl Access> {
114        let conn = match self.config.connection_string {
115            Some(v) => v,
116            None => {
117                return Err(
118                    Error::new(ErrorKind::ConfigInvalid, "connection_string is empty")
119                        .with_context("service", Scheme::Mysql),
120                );
121            }
122        };
123
124        let config = conn.parse::<MySqlConnectOptions>().map_err(|err| {
125            Error::new(ErrorKind::ConfigInvalid, "connection_string is invalid")
126                .with_context("service", Scheme::Mysql)
127                .set_source(err)
128        })?;
129
130        let table = match self.config.table {
131            Some(v) => v,
132            None => {
133                return Err(Error::new(ErrorKind::ConfigInvalid, "table is empty")
134                    .with_context("service", Scheme::Mysql));
135            }
136        };
137
138        let key_field = self.config.key_field.unwrap_or_else(|| "key".to_string());
139
140        let value_field = self
141            .config
142            .value_field
143            .unwrap_or_else(|| "value".to_string());
144
145        let root = normalize_root(self.config.root.unwrap_or_else(|| "/".to_string()).as_str());
146
147        Ok(MysqlBackend::new(MysqlCore {
148            pool: OnceCell::new(),
149            config,
150            table,
151            key_field,
152            value_field,
153        })
154        .with_normalized_root(root))
155    }
156}
157
158/// Backend for mysql service
159#[derive(Clone, Debug)]
160pub struct MysqlBackend {
161    core: Arc<MysqlCore>,
162    root: String,
163    info: Arc<AccessorInfo>,
164}
165
166impl MysqlBackend {
167    pub fn new(core: MysqlCore) -> Self {
168        let info = AccessorInfo::default();
169        info.set_scheme(Scheme::Mysql.into_static());
170        info.set_name(&core.table);
171        info.set_root("/");
172        info.set_native_capability(Capability {
173            read: true,
174            stat: true,
175            write: true,
176            write_can_empty: true,
177            delete: true,
178            shared: true,
179            ..Default::default()
180        });
181
182        Self {
183            core: Arc::new(core),
184            root: "/".to_string(),
185            info: Arc::new(info),
186        }
187    }
188
189    fn with_normalized_root(mut self, root: String) -> Self {
190        self.info.set_root(&root);
191        self.root = root;
192        self
193    }
194}
195
196impl Access for MysqlBackend {
197    type Reader = Buffer;
198    type Writer = MysqlWriter;
199    type Lister = ();
200    type Deleter = oio::OneShotDeleter<MysqlDeleter>;
201
202    fn info(&self) -> Arc<AccessorInfo> {
203        self.info.clone()
204    }
205
206    async fn stat(&self, path: &str, _: OpStat) -> Result<RpStat> {
207        let p = build_abs_path(&self.root, path);
208
209        if p == build_abs_path(&self.root, "") {
210            Ok(RpStat::new(Metadata::new(EntryMode::DIR)))
211        } else {
212            let bs = self.core.get(&p).await?;
213            match bs {
214                Some(bs) => Ok(RpStat::new(
215                    Metadata::new(EntryMode::FILE).with_content_length(bs.len() as u64),
216                )),
217                None => Err(Error::new(ErrorKind::NotFound, "kv not found in mysql")),
218            }
219        }
220    }
221
222    async fn read(&self, path: &str, args: OpRead) -> Result<(RpRead, Self::Reader)> {
223        let p = build_abs_path(&self.root, path);
224        let bs = match self.core.get(&p).await? {
225            Some(bs) => bs,
226            None => return Err(Error::new(ErrorKind::NotFound, "kv not found in mysql")),
227        };
228        Ok((RpRead::new(), bs.slice(args.range().to_range_as_usize())))
229    }
230
231    async fn write(&self, path: &str, _: OpWrite) -> Result<(RpWrite, Self::Writer)> {
232        let p = build_abs_path(&self.root, path);
233        Ok((RpWrite::new(), MysqlWriter::new(self.core.clone(), p)))
234    }
235
236    async fn delete(&self) -> Result<(RpDelete, Self::Deleter)> {
237        Ok((
238            RpDelete::default(),
239            oio::OneShotDeleter::new(MysqlDeleter::new(self.core.clone(), self.root.clone())),
240        ))
241    }
242
243    async fn list(&self, path: &str, _: OpList) -> Result<(RpList, Self::Lister)> {
244        let _ = build_abs_path(&self.root, path);
245        Ok((RpList::default(), ()))
246    }
247}
```
