# 

opendal/services/postgresql/

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
22use sqlx::postgres::PgConnectOptions;
23use tokio::sync::OnceCell;
24
25use super::config::PostgresqlConfig;
26use super::core::*;
27use super::deleter::PostgresqlDeleter;
28use super::writer::PostgresqlWriter;
29use crate::raw::*;
30use crate::*;
31
32/// [PostgreSQL](https://www.postgresql.org/) services support.
33#[doc = include_str!("docs.md")]
34#[derive(Default)]
35pub struct PostgresqlBuilder {
36    pub(super) config: PostgresqlConfig,
37}
38
39impl Debug for PostgresqlBuilder {
40    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
41        let mut d = f.debug_struct("PostgresqlBuilder");
42
43        d.field("config", &self.config);
44        d.finish()
45    }
46}
47
48impl PostgresqlBuilder {
49    /// Set the connection url string of the postgresql service.
50    ///
51    /// The URL should be with a scheme of either `postgres://` or `postgresql://`.
52    ///
53    /// - `postgresql://user@localhost`
54    /// - `postgresql://user:password@%2Fvar%2Flib%2Fpostgresql/mydb?connect_timeout=10`
55    /// - `postgresql://user@host1:1234,host2,host3:5678?target_session_attrs=read-write`
56    /// - `postgresql:///mydb?user=user&host=/var/lib/postgresql`
57    ///
58    /// For more information, please visit <https://docs.rs/sqlx/latest/sqlx/postgres/struct.PgConnectOptions.html>.
59    pub fn connection_string(mut self, v: &str) -> Self {
60        if !v.is_empty() {
61            self.config.connection_string = Some(v.to_string());
62        }
63        self
64    }
65
66    /// Set the working directory, all operations will be performed under it.
67    ///
68    /// default: "/"
69    pub fn root(mut self, root: &str) -> Self {
70        self.config.root = if root.is_empty() {
71            None
72        } else {
73            Some(root.to_string())
74        };
75
76        self
77    }
78
79    /// Set the table name of the postgresql service to read/write.
80    pub fn table(mut self, table: &str) -> Self {
81        if !table.is_empty() {
82            self.config.table = Some(table.to_string());
83        }
84        self
85    }
86
87    /// Set the key field name of the postgresql service to read/write.
88    ///
89    /// Default to `key` if not specified.
90    pub fn key_field(mut self, key_field: &str) -> Self {
91        if !key_field.is_empty() {
92            self.config.key_field = Some(key_field.to_string());
93        }
94        self
95    }
96
97    /// Set the value field name of the postgresql service to read/write.
98    ///
99    /// Default to `value` if not specified.
100    pub fn value_field(mut self, value_field: &str) -> Self {
101        if !value_field.is_empty() {
102            self.config.value_field = Some(value_field.to_string());
103        }
104        self
105    }
106}
107
108impl Builder for PostgresqlBuilder {
109    type Config = PostgresqlConfig;
110
111    fn build(self) -> Result<impl Access> {
112        let conn = match self.config.connection_string {
113            Some(v) => v,
114            None => {
115                return Err(
116                    Error::new(ErrorKind::ConfigInvalid, "connection_string is empty")
117                        .with_context("service", Scheme::Postgresql),
118                );
119            }
120        };
121
122        let config = conn.parse::<PgConnectOptions>().map_err(|err| {
123            Error::new(ErrorKind::ConfigInvalid, "connection_string is invalid")
124                .with_context("service", Scheme::Postgresql)
125                .set_source(err)
126        })?;
127
128        let table = match self.config.table {
129            Some(v) => v,
130            None => {
131                return Err(Error::new(ErrorKind::ConfigInvalid, "table is empty")
132                    .with_context("service", Scheme::Postgresql));
133            }
134        };
135
136        let key_field = self.config.key_field.unwrap_or_else(|| "key".to_string());
137
138        let value_field = self
139            .config
140            .value_field
141            .unwrap_or_else(|| "value".to_string());
142
143        let root = normalize_root(self.config.root.unwrap_or_else(|| "/".to_string()).as_str());
144
145        Ok(PostgresqlBackend::new(PostgresqlCore {
146            pool: OnceCell::new(),
147            config,
148            table,
149            key_field,
150            value_field,
151        })
152        .with_normalized_root(root))
153    }
154}
155
156/// Backend for Postgresql service
157#[derive(Clone, Debug)]
158pub struct PostgresqlBackend {
159    core: Arc<PostgresqlCore>,
160    root: String,
161    info: Arc<AccessorInfo>,
162}
163
164impl PostgresqlBackend {
165    pub fn new(core: PostgresqlCore) -> Self {
166        let info = AccessorInfo::default();
167        info.set_scheme(Scheme::Postgresql.into_static());
168        info.set_name(&core.table);
169        info.set_root("/");
170        info.set_native_capability(Capability {
171            read: true,
172            stat: true,
173            write: true,
174            write_can_empty: true,
175            delete: true,
176            shared: true,
177            ..Default::default()
178        });
179
180        Self {
181            core: Arc::new(core),
182            root: "/".to_string(),
183            info: Arc::new(info),
184        }
185    }
186
187    fn with_normalized_root(mut self, root: String) -> Self {
188        self.info.set_root(&root);
189        self.root = root;
190        self
191    }
192}
193
194impl Access for PostgresqlBackend {
195    type Reader = Buffer;
196    type Writer = PostgresqlWriter;
197    type Lister = ();
198    type Deleter = oio::OneShotDeleter<PostgresqlDeleter>;
199
200    fn info(&self) -> Arc<AccessorInfo> {
201        self.info.clone()
202    }
203
204    async fn stat(&self, path: &str, _: OpStat) -> Result<RpStat> {
205        let p = build_abs_path(&self.root, path);
206
207        if p == build_abs_path(&self.root, "") {
208            Ok(RpStat::new(Metadata::new(EntryMode::DIR)))
209        } else {
210            let bs = self.core.get(&p).await?;
211            match bs {
212                Some(bs) => Ok(RpStat::new(
213                    Metadata::new(EntryMode::FILE).with_content_length(bs.len() as u64),
214                )),
215                None => Err(Error::new(
216                    ErrorKind::NotFound,
217                    "kv not found in postgresql",
218                )),
219            }
220        }
221    }
222
223    async fn read(&self, path: &str, args: OpRead) -> Result<(RpRead, Self::Reader)> {
224        let p = build_abs_path(&self.root, path);
225        let bs = match self.core.get(&p).await? {
226            Some(bs) => bs,
227            None => {
228                return Err(Error::new(
229                    ErrorKind::NotFound,
230                    "kv not found in postgresql",
231                ));
232            }
233        };
234        Ok((RpRead::new(), bs.slice(args.range().to_range_as_usize())))
235    }
236
237    async fn write(&self, path: &str, _: OpWrite) -> Result<(RpWrite, Self::Writer)> {
238        let p = build_abs_path(&self.root, path);
239        Ok((RpWrite::new(), PostgresqlWriter::new(self.core.clone(), p)))
240    }
241
242    async fn delete(&self) -> Result<(RpDelete, Self::Deleter)> {
243        Ok((
244            RpDelete::default(),
245            oio::OneShotDeleter::new(PostgresqlDeleter::new(self.core.clone(), self.root.clone())),
246        ))
247    }
248
249    async fn list(&self, path: &str, _: OpList) -> Result<(RpList, Self::Lister)> {
250        let _ = build_abs_path(&self.root, path);
251        Ok((RpList::default(), ()))
252    }
253}
```
