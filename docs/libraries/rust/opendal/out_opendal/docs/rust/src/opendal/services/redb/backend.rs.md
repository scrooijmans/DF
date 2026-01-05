# 

opendal/services/redb/

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
22use crate::Builder;
23use crate::Error;
24use crate::ErrorKind;
25use crate::Scheme;
26use crate::raw::adapters::kv;
27use crate::raw::*;
28use crate::services::RedbConfig;
29use crate::*;
30
31/// Redb service support.
32#[doc = include_str!("docs.md")]
33#[derive(Default, Debug)]
34pub struct RedbBuilder {
35    pub(super) config: RedbConfig,
36
37    pub(super) database: Option<Arc<redb::Database>>,
38}
39
40impl RedbBuilder {
41    /// Set the database for Redb.
42    ///
43    /// This method should be called when you want to
44    /// use multiple tables of one database because
45    /// Redb doesn't allow opening a database that have been opened.
46    ///
47    /// <div class="warning">
48    ///
49    /// `datadir` and `database` should not be set simultaneously.
50    /// If both are set, `database` will take precedence.
51    ///
52    /// </div>
53    pub fn database(mut self, db: Arc<redb::Database>) -> Self {
54        self.database = Some(db);
55        self
56    }
57
58    /// Set the path to the redb data directory. Will create if not exists.
59    ///
60    ///
61    /// <div class="warning">
62    ///
63    /// Opening redb database via `datadir` takes away the ability to access multiple redb tables.
64    /// If you need to access multiple redb tables, the correct solution is to
65    /// create an `Arc<redb::database>` beforehand and then share it via [`database`]
66    /// with multiple builders where every builder will open one redb table.
67    ///
68    /// </div>
69    ///
70    /// [`database`]: RedbBuilder::database
71    pub fn datadir(mut self, path: &str) -> Self {
72        self.config.datadir = Some(path.into());
73        self
74    }
75
76    /// Set the table name for Redb. Will create if not exists.
77    pub fn table(mut self, table: &str) -> Self {
78        self.config.table = Some(table.into());
79        self
80    }
81
82    /// Set the root for Redb.
83    pub fn root(mut self, path: &str) -> Self {
84        self.config.root = Some(path.into());
85        self
86    }
87}
88
89impl Builder for RedbBuilder {
90    type Config = RedbConfig;
91
92    fn build(self) -> Result<impl Access> {
93        let table_name = self.config.table.ok_or_else(|| {
94            Error::new(ErrorKind::ConfigInvalid, "table is required but not set")
95                .with_context("service", Scheme::Redb)
96        })?;
97
98        let (datadir, db) = if let Some(db) = self.database {
99            (None, db)
100        } else {
101            let datadir = self.config.datadir.ok_or_else(|| {
102                Error::new(ErrorKind::ConfigInvalid, "datadir is required but not set")
103                    .with_context("service", Scheme::Redb)
104            })?;
105
106            let db = redb::Database::create(&datadir)
107                .map_err(parse_database_error)?
108                .into();
109
110            (Some(datadir), db)
111        };
112
113        create_table(&db, &table_name)?;
114
115        Ok(RedbBackend::new(Adapter {
116            datadir,
117            table: table_name,
118            db,
119        })
120        .with_root(self.config.root.as_deref().unwrap_or_default()))
121    }
122}
123
124/// Backend for Redb services.
125pub type RedbBackend = kv::Backend<Adapter>;
126
127#[derive(Clone)]
128pub struct Adapter {
129    datadir: Option<String>,
130    table: String,
131    db: Arc<redb::Database>,
132}
133
134impl Debug for Adapter {
135    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
136        let mut ds = f.debug_struct("Adapter");
137        ds.field("path", &self.datadir);
138        ds.finish()
139    }
140}
141
142impl kv::Adapter for Adapter {
143    type Scanner = ();
144
145    fn info(&self) -> kv::Info {
146        kv::Info::new(
147            Scheme::Redb,
148            &self.table,
149            Capability {
150                read: true,
151                write: true,
152                shared: false,
153                ..Default::default()
154            },
155        )
156    }
157
158    async fn get(&self, path: &str) -> Result<Option<Buffer>> {
159        let read_txn = self.db.begin_read().map_err(parse_transaction_error)?;
160
161        let table_define: redb::TableDefinition<&str, &[u8]> =
162            redb::TableDefinition::new(&self.table);
163
164        let table = read_txn
165            .open_table(table_define)
166            .map_err(parse_table_error)?;
167
168        let result = match table.get(path) {
169            Ok(Some(v)) => Ok(Some(v.value().to_vec())),
170            Ok(None) => Ok(None),
171            Err(e) => Err(parse_storage_error(e)),
172        }?;
173        Ok(result.map(Buffer::from))
174    }
175
176    async fn set(&self, path: &str, value: Buffer) -> Result<()> {
177        let write_txn = self.db.begin_write().map_err(parse_transaction_error)?;
178
179        let table_define: redb::TableDefinition<&str, &[u8]> =
180            redb::TableDefinition::new(&self.table);
181
182        {
183            let mut table = write_txn
184                .open_table(table_define)
185                .map_err(parse_table_error)?;
186
187            table
188                .insert(path, &*value.to_vec())
189                .map_err(parse_storage_error)?;
190        }
191
192        write_txn.commit().map_err(parse_commit_error)?;
193        Ok(())
194    }
195
196    async fn delete(&self, path: &str) -> Result<()> {
197        let write_txn = self.db.begin_write().map_err(parse_transaction_error)?;
198
199        let table_define: redb::TableDefinition<&str, &[u8]> =
200            redb::TableDefinition::new(&self.table);
201
202        {
203            let mut table = write_txn
204                .open_table(table_define)
205                .map_err(parse_table_error)?;
206
207            table.remove(path).map_err(parse_storage_error)?;
208        }
209
210        write_txn.commit().map_err(parse_commit_error)?;
211        Ok(())
212    }
213}
214
215fn parse_transaction_error(e: redb::TransactionError) -> Error {
216    Error::new(ErrorKind::Unexpected, "error from redb").set_source(e)
217}
218
219fn parse_table_error(e: redb::TableError) -> Error {
220    match e {
221        redb::TableError::TableDoesNotExist(_) => {
222            Error::new(ErrorKind::NotFound, "error from redb").set_source(e)
223        }
224        _ => Error::new(ErrorKind::Unexpected, "error from redb").set_source(e),
225    }
226}
227
228fn parse_storage_error(e: redb::StorageError) -> Error {
229    Error::new(ErrorKind::Unexpected, "error from redb").set_source(e)
230}
231
232fn parse_database_error(e: redb::DatabaseError) -> Error {
233    Error::new(ErrorKind::Unexpected, "error from redb").set_source(e)
234}
235
236fn parse_commit_error(e: redb::CommitError) -> Error {
237    Error::new(ErrorKind::Unexpected, "error from redb").set_source(e)
238}
239
240/// Check if a table exists, otherwise create it.
241fn create_table(db: &redb::Database, table: &str) -> Result<()> {
242    // Only one `WriteTransaction` is permitted at same time,
243    // applying new one will block until it available.
244    //
245    // So we first try checking table existence via `ReadTransaction`.
246    {
247        let read_txn = db.begin_read().map_err(parse_transaction_error)?;
248
249        let table_define: redb::TableDefinition<&str, &[u8]> = redb::TableDefinition::new(table);
250
251        match read_txn.open_table(table_define) {
252            Ok(_) => return Ok(()),
253            Err(redb::TableError::TableDoesNotExist(_)) => (),
254            Err(e) => return Err(parse_table_error(e)),
255        }
256    }
257
258    {
259        let write_txn = db.begin_write().map_err(parse_transaction_error)?;
260
261        let table_define: redb::TableDefinition<&str, &[u8]> = redb::TableDefinition::new(table);
262
263        write_txn
264            .open_table(table_define)
265            .map_err(parse_table_error)?;
266        write_txn.commit().map_err(parse_commit_error)?;
267    }
268
269    Ok(())
270}
```
