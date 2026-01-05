# 

opendal/services/sqlite/

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
20use std::pin::Pin;
21use std::str::FromStr;
22use std::task::Context;
23use std::task::Poll;
24
25use futures::Stream;
26use futures::StreamExt;
27use futures::stream::BoxStream;
28use ouroboros::self_referencing;
29use sqlx::SqlitePool;
30use sqlx::sqlite::SqliteConnectOptions;
31use tokio::sync::OnceCell;
32
33use crate::raw::adapters::kv;
34use crate::raw::*;
35use crate::services::SqliteConfig;
36use crate::*;
37
38#[doc = include_str!("docs.md")]
39#[derive(Default)]
40pub struct SqliteBuilder {
41    pub(super) config: SqliteConfig,
42}
43
44impl Debug for SqliteBuilder {
45    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
46        let mut ds = f.debug_struct("SqliteBuilder");
47
48        ds.field("config", &self.config);
49        ds.finish()
50    }
51}
52
53impl SqliteBuilder {
54    /// Set the connection_string of the sqlite service.
55    ///
56    /// This connection string is used to connect to the sqlite service. There are url based formats:
57    ///
58    /// ## Url
59    ///
60    /// This format resembles the url format of the sqlite client:
61    ///
62    /// - `sqlite::memory:`
63    /// - `sqlite:data.db`
64    /// - `sqlite://data.db`
65    ///
66    /// For more information, please visit <https://docs.rs/sqlx/latest/sqlx/sqlite/struct.SqliteConnectOptions.html>.
67    pub fn connection_string(mut self, v: &str) -> Self {
68        if !v.is_empty() {
69            self.config.connection_string = Some(v.to_string());
70        }
71        self
72    }
73
74    /// set the working directory, all operations will be performed under it.
75    ///
76    /// default: "/"
77    pub fn root(mut self, root: &str) -> Self {
78        self.config.root = if root.is_empty() {
79            None
80        } else {
81            Some(root.to_string())
82        };
83
84        self
85    }
86
87    /// Set the table name of the sqlite service to read/write.
88    pub fn table(mut self, table: &str) -> Self {
89        if !table.is_empty() {
90            self.config.table = Some(table.to_string());
91        }
92        self
93    }
94
95    /// Set the key field name of the sqlite service to read/write.
96    ///
97    /// Default to `key` if not specified.
98    pub fn key_field(mut self, key_field: &str) -> Self {
99        if !key_field.is_empty() {
100            self.config.key_field = Some(key_field.to_string());
101        }
102        self
103    }
104
105    /// Set the value field name of the sqlite service to read/write.
106    ///
107    /// Default to `value` if not specified.
108    pub fn value_field(mut self, value_field: &str) -> Self {
109        if !value_field.is_empty() {
110            self.config.value_field = Some(value_field.to_string());
111        }
112        self
113    }
114}
115
116impl Builder for SqliteBuilder {
117    type Config = SqliteConfig;
118
119    fn build(self) -> Result<impl Access> {
120        let conn = match self.config.connection_string {
121            Some(v) => v,
122            None => {
123                return Err(Error::new(
124                    ErrorKind::ConfigInvalid,
125                    "connection_string is required but not set",
126                )
127                .with_context("service", Scheme::Sqlite));
128            }
129        };
130
131        let config = SqliteConnectOptions::from_str(&conn).map_err(|err| {
132            Error::new(ErrorKind::ConfigInvalid, "connection_string is invalid")
133                .with_context("service", Scheme::Sqlite)
134                .set_source(err)
135        })?;
136
137        let table = match self.config.table {
138            Some(v) => v,
139            None => {
140                return Err(Error::new(ErrorKind::ConfigInvalid, "table is empty")
141                    .with_context("service", Scheme::Sqlite));
142            }
143        };
144
145        let key_field = self.config.key_field.unwrap_or_else(|| "key".to_string());
146
147        let value_field = self
148            .config
149            .value_field
150            .unwrap_or_else(|| "value".to_string());
151
152        let root = normalize_root(self.config.root.as_deref().unwrap_or("/"));
153
154        Ok(SqliteBackend::new(Adapter {
155            pool: OnceCell::new(),
156            config,
157            table,
158            key_field,
159            value_field,
160        })
161        .with_normalized_root(root))
162    }
163}
164
165pub type SqliteBackend = kv::Backend<Adapter>;
166
167#[derive(Debug, Clone)]
168pub struct Adapter {
169    pool: OnceCell<SqlitePool>,
170    config: SqliteConnectOptions,
171
172    table: String,
173    key_field: String,
174    value_field: String,
175}
176
177impl Adapter {
178    async fn get_client(&self) -> Result<&SqlitePool> {
179        self.pool
180            .get_or_try_init(|| async {
181                let pool = SqlitePool::connect_with(self.config.clone())
182                    .await
183                    .map_err(parse_sqlite_error)?;
184                Ok(pool)
185            })
186            .await
187    }
188}
189
190#[self_referencing]
191pub struct SqliteScanner {
192    pool: SqlitePool,
193    query: String,
194
195    #[borrows(pool, query)]
196    #[covariant]
197    stream: BoxStream<'this, Result<String>>,
198}
199
200impl Stream for SqliteScanner {
201    type Item = Result<String>;
202
203    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
204        self.with_stream_mut(|s| s.poll_next_unpin(cx))
205    }
206}
207
208unsafe impl Sync for SqliteScanner {}
209
210impl kv::Scan for SqliteScanner {
211    async fn next(&mut self) -> Result<Option<String>> {
212        <Self as StreamExt>::next(self).await.transpose()
213    }
214}
215
216impl kv::Adapter for Adapter {
217    type Scanner = SqliteScanner;
218
219    fn info(&self) -> kv::Info {
220        kv::Info::new(
221            Scheme::Sqlite,
222            &self.table,
223            Capability {
224                read: true,
225                write: true,
226                delete: true,
227                list: true,
228                shared: false,
229                ..Default::default()
230            },
231        )
232    }
233
234    async fn get(&self, path: &str) -> Result<Option<Buffer>> {
235        let pool = self.get_client().await?;
236
237        let value: Option<Vec<u8>> = sqlx::query_scalar(&format!(
238            "SELECT `{}` FROM `{}` WHERE `{}` = $1 LIMIT 1",
239            self.value_field, self.table, self.key_field
240        ))
241        .bind(path)
242        .fetch_optional(pool)
243        .await
244        .map_err(parse_sqlite_error)?;
245
246        Ok(value.map(Buffer::from))
247    }
248
249    async fn set(&self, path: &str, value: Buffer) -> Result<()> {
250        let pool = self.get_client().await?;
251
252        sqlx::query(&format!(
253            "INSERT OR REPLACE INTO `{}` (`{}`, `{}`) VALUES ($1, $2)",
254            self.table, self.key_field, self.value_field,
255        ))
256        .bind(path)
257        .bind(value.to_vec())
258        .execute(pool)
259        .await
260        .map_err(parse_sqlite_error)?;
261
262        Ok(())
263    }
264
265    async fn delete(&self, path: &str) -> Result<()> {
266        let pool = self.get_client().await?;
267
268        sqlx::query(&format!(
269            "DELETE FROM `{}` WHERE `{}` = $1",
270            self.table, self.key_field
271        ))
272        .bind(path)
273        .execute(pool)
274        .await
275        .map_err(parse_sqlite_error)?;
276
277        Ok(())
278    }
279
280    async fn scan(&self, path: &str) -> Result<Self::Scanner> {
281        let pool = self.get_client().await?;
282        let stream = SqliteScannerBuilder {
283            pool: pool.clone(),
284            query: format!(
285                "SELECT `{}` FROM `{}` WHERE `{}` LIKE $1",
286                self.key_field, self.table, self.key_field
287            ),
288            stream_builder: |pool, query| {
289                sqlx::query_scalar(query)
290                    .bind(format!("{path}%"))
291                    .fetch(pool)
292                    .map(|v| v.map_err(parse_sqlite_error))
293                    .boxed()
294            },
295        }
296        .build();
297
298        Ok(stream)
299    }
300}
301
302fn parse_sqlite_error(err: sqlx::Error) -> Error {
303    let is_temporary = matches!(
304        &err,
305        sqlx::Error::Database(db_err) if db_err.code().is_some_and(|c| c == "5" || c == "6")
306    );
307
308    let message = if is_temporary {
309        "database is locked or busy"
310    } else {
311        "unhandled error from sqlite"
312    };
313
314    let mut error = Error::new(ErrorKind::Unexpected, message).set_source(err);
315    if is_temporary {
316        error = error.set_temporary();
317    }
318    error
319}
```
