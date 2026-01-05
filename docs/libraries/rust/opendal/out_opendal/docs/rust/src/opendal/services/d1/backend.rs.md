# 

opendal/services/d1/

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
20use super::config::D1Config;
21use super::core::*;
22use super::deleter::D1Deleter;
23use super::writer::D1Writer;
24use crate::raw::*;
25use crate::*;
26
27#[doc = include_str!("docs.md")]
28#[derive(Default)]
29pub struct D1Builder {
30    pub(super) config: D1Config,
31
32    pub(super) http_client: Option<HttpClient>,
33}
34
35impl D1Builder {
36    /// Set api token for the cloudflare d1 service.
37    ///
38    /// create a api token from [here](https://dash.cloudflare.com/profile/api-tokens)
39    pub fn token(mut self, token: &str) -> Self {
40        if !token.is_empty() {
41            self.config.token = Some(token.to_string());
42        }
43        self
44    }
45
46    /// Set the account identifier for the cloudflare d1 service.
47    ///
48    /// get the account identifier from Workers & Pages -> Overview -> Account ID
49    /// If not specified, it will return an error when building.
50    pub fn account_id(mut self, account_id: &str) -> Self {
51        if !account_id.is_empty() {
52            self.config.account_id = Some(account_id.to_string());
53        }
54        self
55    }
56
57    /// Set the database identifier for the cloudflare d1 service.
58    ///
59    /// get the database identifier from Workers & Pages -> D1 -> [Your Database] -> Database ID
60    /// If not specified, it will return an error when building.
61    pub fn database_id(mut self, database_id: &str) -> Self {
62        if !database_id.is_empty() {
63            self.config.database_id = Some(database_id.to_string());
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
81    /// Set the table name of the d1 service to read/write.
82    ///
83    /// If not specified, it will return an error when building.
84    pub fn table(mut self, table: &str) -> Self {
85        if !table.is_empty() {
86            self.config.table = Some(table.to_owned());
87        }
88        self
89    }
90
91    /// Set the key field name of the d1 service to read/write.
92    ///
93    /// Default to `key` if not specified.
94    pub fn key_field(mut self, key_field: &str) -> Self {
95        if !key_field.is_empty() {
96            self.config.key_field = Some(key_field.to_string());
97        }
98        self
99    }
100
101    /// Set the value field name of the d1 service to read/write.
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
112impl Builder for D1Builder {
113    type Config = D1Config;
114
115    fn build(self) -> Result<impl Access> {
116        let mut authorization = None;
117        let config = self.config;
118
119        if let Some(token) = config.token {
120            authorization = Some(format_authorization_by_bearer(&token)?)
121        }
122
123        let Some(account_id) = config.account_id else {
124            return Err(Error::new(
125                ErrorKind::ConfigInvalid,
126                "account_id is required",
127            ));
128        };
129
130        let Some(database_id) = config.database_id.clone() else {
131            return Err(Error::new(
132                ErrorKind::ConfigInvalid,
133                "database_id is required",
134            ));
135        };
136
137        let client = if let Some(client) = self.http_client {
138            client
139        } else {
140            HttpClient::new().map_err(|err| {
141                err.with_operation("Builder::build")
142                    .with_context("service", Scheme::D1)
143            })?
144        };
145
146        let Some(table) = config.table.clone() else {
147            return Err(Error::new(ErrorKind::ConfigInvalid, "table is required"));
148        };
149
150        let key_field = config
151            .key_field
152            .clone()
153            .unwrap_or_else(|| "key".to_string());
154
155        let value_field = config
156            .value_field
157            .clone()
158            .unwrap_or_else(|| "value".to_string());
159
160        let root = normalize_root(
161            config
162                .root
163                .clone()
164                .unwrap_or_else(|| "/".to_string())
165                .as_str(),
166        );
167        Ok(D1Backend::new(D1Core {
168            authorization,
169            account_id,
170            database_id,
171            client,
172            table,
173            key_field,
174            value_field,
175        })
176        .with_normalized_root(root))
177    }
178}
179
180/// Backend for D1 services.
181#[derive(Clone, Debug)]
182pub struct D1Backend {
183    core: Arc<D1Core>,
184    root: String,
185    info: Arc<AccessorInfo>,
186}
187
188impl D1Backend {
189    pub fn new(core: D1Core) -> Self {
190        let info = AccessorInfo::default();
191        info.set_scheme(Scheme::D1.into_static());
192        info.set_name(&core.table);
193        info.set_root("/");
194        info.set_native_capability(Capability {
195            read: true,
196            stat: true,
197            write: true,
198            write_can_empty: true,
199            // Cloudflare D1 supports 1MB as max in write_total.
200            // refer to https://developers.cloudflare.com/d1/platform/limits/
201            write_total_max_size: Some(1000 * 1000),
202            delete: true,
203            shared: true,
204            ..Default::default()
205        });
206
207        Self {
208            core: Arc::new(core),
209            root: "/".to_string(),
210            info: Arc::new(info),
211        }
212    }
213
214    fn with_normalized_root(mut self, root: String) -> Self {
215        self.info.set_root(&root);
216        self.root = root;
217        self
218    }
219}
220
221impl Access for D1Backend {
222    type Reader = Buffer;
223    type Writer = D1Writer;
224    type Lister = ();
225    type Deleter = oio::OneShotDeleter<D1Deleter>;
226
227    fn info(&self) -> Arc<AccessorInfo> {
228        self.info.clone()
229    }
230
231    async fn stat(&self, path: &str, _: OpStat) -> Result<RpStat> {
232        let p = build_abs_path(&self.root, path);
233
234        if p == build_abs_path(&self.root, "") {
235            Ok(RpStat::new(Metadata::new(EntryMode::DIR)))
236        } else {
237            let bs = self.core.get(&p).await?;
238            match bs {
239                Some(bs) => Ok(RpStat::new(
240                    Metadata::new(EntryMode::FILE).with_content_length(bs.len() as u64),
241                )),
242                None => Err(Error::new(ErrorKind::NotFound, "kv not found in d1")),
243            }
244        }
245    }
246
247    async fn read(&self, path: &str, args: OpRead) -> Result<(RpRead, Self::Reader)> {
248        let p = build_abs_path(&self.root, path);
249        let bs = match self.core.get(&p).await? {
250            Some(bs) => bs,
251            None => {
252                return Err(Error::new(ErrorKind::NotFound, "kv not found in d1"));
253            }
254        };
255        Ok((RpRead::new(), bs.slice(args.range().to_range_as_usize())))
256    }
257
258    async fn write(&self, path: &str, _: OpWrite) -> Result<(RpWrite, Self::Writer)> {
259        let p = build_abs_path(&self.root, path);
260        Ok((RpWrite::new(), D1Writer::new(self.core.clone(), p)))
261    }
262
263    async fn delete(&self) -> Result<(RpDelete, Self::Deleter)> {
264        Ok((
265            RpDelete::default(),
266            oio::OneShotDeleter::new(D1Deleter::new(self.core.clone(), self.root.clone())),
267        ))
268    }
269
270    async fn list(&self, path: &str, _: OpList) -> Result<(RpList, Self::Lister)> {
271        let _ = build_abs_path(&self.root, path);
272        Ok((RpList::default(), ()))
273    }
274}
```
