# 

opendal/services/dbfs/

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
22use bytes::Buf;
23use http::StatusCode;
24use log::debug;
25use serde::Deserialize;
26
27use super::DBFS_SCHEME;
28use super::core::DbfsCore;
29use super::delete::DbfsDeleter;
30use super::error::parse_error;
31use super::lister::DbfsLister;
32use super::writer::DbfsWriter;
33use crate::raw::*;
34use crate::services::DbfsConfig;
35use crate::*;
36
37/// [Dbfs](https://docs.databricks.com/api/azure/workspace/dbfs)'s REST API support.
38#[doc = include_str!("docs.md")]
39#[derive(Default, Clone)]
40pub struct DbfsBuilder {
41    pub(super) config: DbfsConfig,
42}
43
44impl Debug for DbfsBuilder {
45    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
46        let mut ds = f.debug_struct("DbfsBuilder");
47
48        ds.field("config", &self.config);
49
50        ds.finish()
51    }
52}
53
54impl DbfsBuilder {
55    /// Set root of this backend.
56    ///
57    /// All operations will happen under this root.
58    pub fn root(mut self, root: &str) -> Self {
59        self.config.root = if root.is_empty() {
60            None
61        } else {
62            Some(root.to_string())
63        };
64
65        self
66    }
67
68    /// Set endpoint of this backend.
69    ///
70    /// Endpoint must be full uri, e.g.
71    ///
72    /// - Azure: `https://adb-1234567890123456.78.azuredatabricks.net`
73    /// - Aws: `https://dbc-123a5678-90bc.cloud.databricks.com`
74    pub fn endpoint(mut self, endpoint: &str) -> Self {
75        self.config.endpoint = if endpoint.is_empty() {
76            None
77        } else {
78            Some(endpoint.trim_end_matches('/').to_string())
79        };
80        self
81    }
82
83    /// Set the token of this backend.
84    pub fn token(mut self, token: &str) -> Self {
85        if !token.is_empty() {
86            self.config.token = Some(token.to_string());
87        }
88        self
89    }
90}
91
92impl Builder for DbfsBuilder {
93    type Config = DbfsConfig;
94
95    /// Build a DbfsBackend.
96    fn build(self) -> Result<impl Access> {
97        debug!("backend build started: {:?}", &self);
98
99        let root = normalize_root(&self.config.root.unwrap_or_default());
100        debug!("backend use root {root}");
101
102        let endpoint = match &self.config.endpoint {
103            Some(endpoint) => Ok(endpoint.clone()),
104            None => Err(Error::new(ErrorKind::ConfigInvalid, "endpoint is empty")
105                .with_operation("Builder::build")
106                .with_context("service", Scheme::Dbfs)),
107        }?;
108        debug!("backend use endpoint: {}", &endpoint);
109
110        let token = match self.config.token {
111            Some(token) => token,
112            None => {
113                return Err(Error::new(
114                    ErrorKind::ConfigInvalid,
115                    "missing token for Dbfs",
116                ));
117            }
118        };
119
120        let client = HttpClient::new()?;
121        Ok(DbfsBackend {
122            core: Arc::new(DbfsCore {
123                root,
124                endpoint: endpoint.to_string(),
125                token,
126                client,
127            }),
128        })
129    }
130}
131
132/// Backend for DBFS service
133#[derive(Debug, Clone)]
134pub struct DbfsBackend {
135    core: Arc<DbfsCore>,
136}
137
138impl Access for DbfsBackend {
139    type Reader = ();
140    type Writer = oio::OneShotWriter<DbfsWriter>;
141    type Lister = oio::PageLister<DbfsLister>;
142    type Deleter = oio::OneShotDeleter<DbfsDeleter>;
143
144    fn info(&self) -> Arc<AccessorInfo> {
145        let am = AccessorInfo::default();
146        am.set_scheme(DBFS_SCHEME)
147            .set_root(&self.core.root)
148            .set_native_capability(Capability {
149                stat: true,
150
151                write: true,
152                create_dir: true,
153                delete: true,
154                rename: true,
155
156                list: true,
157
158                shared: true,
159
160                ..Default::default()
161            });
162        am.into()
163    }
164
165    async fn create_dir(&self, path: &str, _: OpCreateDir) -> Result<RpCreateDir> {
166        let resp = self.core.dbfs_create_dir(path).await?;
167
168        let status = resp.status();
169
170        match status {
171            StatusCode::CREATED | StatusCode::OK => Ok(RpCreateDir::default()),
172            _ => Err(parse_error(resp)),
173        }
174    }
175
176    async fn stat(&self, path: &str, _: OpStat) -> Result<RpStat> {
177        // Stat root always returns a DIR.
178        if path == "/" {
179            return Ok(RpStat::new(Metadata::new(EntryMode::DIR)));
180        }
181
182        let resp = self.core.dbfs_get_status(path).await?;
183
184        let status = resp.status();
185
186        match status {
187            StatusCode::OK => {
188                let mut meta = parse_into_metadata(path, resp.headers())?;
189                let bs = resp.into_body();
190                let decoded_response: DbfsStatus =
191                    serde_json::from_reader(bs.reader()).map_err(new_json_deserialize_error)?;
192                meta.set_last_modified(Timestamp::from_millisecond(
193                    decoded_response.modification_time,
194                )?);
195                match decoded_response.is_dir {
196                    true => meta.set_mode(EntryMode::DIR),
197                    false => {
198                        meta.set_mode(EntryMode::FILE);
199                        meta.set_content_length(decoded_response.file_size as u64)
200                    }
201                };
202                Ok(RpStat::new(meta))
203            }
204            StatusCode::NOT_FOUND if path.ends_with('/') => {
205                Ok(RpStat::new(Metadata::new(EntryMode::DIR)))
206            }
207            _ => Err(parse_error(resp)),
208        }
209    }
210
211    async fn write(&self, path: &str, args: OpWrite) -> Result<(RpWrite, Self::Writer)> {
212        Ok((
213            RpWrite::default(),
214            oio::OneShotWriter::new(DbfsWriter::new(self.core.clone(), args, path.to_string())),
215        ))
216    }
217
218    async fn delete(&self) -> Result<(RpDelete, Self::Deleter)> {
219        Ok((
220            RpDelete::default(),
221            oio::OneShotDeleter::new(DbfsDeleter::new(self.core.clone())),
222        ))
223    }
224
225    async fn list(&self, path: &str, _args: OpList) -> Result<(RpList, Self::Lister)> {
226        let l = DbfsLister::new(self.core.clone(), path.to_string());
227
228        Ok((RpList::default(), oio::PageLister::new(l)))
229    }
230
231    async fn rename(&self, from: &str, to: &str, _args: OpRename) -> Result<RpRename> {
232        self.core.dbfs_ensure_parent_path(to).await?;
233
234        let resp = self.core.dbfs_rename(from, to).await?;
235
236        let status = resp.status();
237
238        match status {
239            StatusCode::OK => Ok(RpRename::default()),
240            _ => Err(parse_error(resp)),
241        }
242    }
243}
244
245#[derive(Deserialize)]
246struct DbfsStatus {
247    // Not used fields.
248    // path: String,
249    is_dir: bool,
250    file_size: i64,
251    modification_time: i64,
252}
```
