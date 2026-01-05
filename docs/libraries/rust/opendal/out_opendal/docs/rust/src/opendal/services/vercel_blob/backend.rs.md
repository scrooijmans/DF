# 

opendal/services/vercel_blob/

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
23use http::Response;
24use http::StatusCode;
25use log::debug;
26
27use super::VERCEL_BLOB_SCHEME;
28use super::core::Blob;
29use super::core::VercelBlobCore;
30use super::core::parse_blob;
31use super::delete::VercelBlobDeleter;
32use super::error::parse_error;
33use super::lister::VercelBlobLister;
34use super::writer::VercelBlobWriter;
35use super::writer::VercelBlobWriters;
36use crate::raw::*;
37use crate::services::VercelBlobConfig;
38use crate::*;
39
40/// [VercelBlob](https://vercel.com/docs/storage/vercel-blob) services support.
41#[doc = include_str!("docs.md")]
42#[derive(Default)]
43pub struct VercelBlobBuilder {
44    pub(super) config: VercelBlobConfig,
45
46    #[deprecated(since = "0.53.0", note = "Use `Operator::update_http_client` instead")]
47    pub(super) http_client: Option<HttpClient>,
48}
49
50impl Debug for VercelBlobBuilder {
51    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
52        let mut d = f.debug_struct("VercelBlobBuilder");
53
54        d.field("config", &self.config);
55        d.finish_non_exhaustive()
56    }
57}
58
59impl VercelBlobBuilder {
60    /// Set root of this backend.
61    ///
62    /// All operations will happen under this root.
63    pub fn root(mut self, root: &str) -> Self {
64        self.config.root = if root.is_empty() {
65            None
66        } else {
67            Some(root.to_string())
68        };
69
70        self
71    }
72
73    /// Vercel Blob token.
74    ///
75    /// Get from Vercel environment variable `BLOB_READ_WRITE_TOKEN`.
76    /// It is required.
77    pub fn token(mut self, token: &str) -> Self {
78        if !token.is_empty() {
79            self.config.token = Some(token.to_string());
80        }
81        self
82    }
83
84    /// Specify the http client that used by this service.
85    ///
86    /// # Notes
87    ///
88    /// This API is part of OpenDAL's Raw API. `HttpClient` could be changed
89    /// during minor updates.
90    #[deprecated(since = "0.53.0", note = "Use `Operator::update_http_client` instead")]
91    #[allow(deprecated)]
92    pub fn http_client(mut self, client: HttpClient) -> Self {
93        self.http_client = Some(client);
94        self
95    }
96}
97
98impl Builder for VercelBlobBuilder {
99    type Config = VercelBlobConfig;
100
101    /// Builds the backend and returns the result of VercelBlobBackend.
102    fn build(self) -> Result<impl Access> {
103        debug!("backend build started: {:?}", &self);
104
105        let root = normalize_root(&self.config.root.clone().unwrap_or_default());
106        debug!("backend use root {}", &root);
107
108        // Handle token.
109        let Some(token) = self.config.token.clone() else {
110            return Err(Error::new(ErrorKind::ConfigInvalid, "token is empty")
111                .with_operation("Builder::build")
112                .with_context("service", Scheme::VercelBlob));
113        };
114
115        Ok(VercelBlobBackend {
116            core: Arc::new(VercelBlobCore {
117                info: {
118                    let am = AccessorInfo::default();
119                    am.set_scheme(VERCEL_BLOB_SCHEME)
120                        .set_root(&root)
121                        .set_native_capability(Capability {
122                            stat: true,
123
124                            read: true,
125
126                            write: true,
127                            write_can_empty: true,
128                            write_can_multi: true,
129                            write_multi_min_size: Some(5 * 1024 * 1024),
130
131                            copy: true,
132
133                            list: true,
134                            list_with_limit: true,
135
136                            delete: true,
137
138                            shared: true,
139
140                            ..Default::default()
141                        });
142
143                    // allow deprecated api here for compatibility
144                    #[allow(deprecated)]
145                    if let Some(client) = self.http_client {
146                        am.update_http_client(|_| client);
147                    }
148
149                    am.into()
150                },
151                root,
152                token,
153            }),
154        })
155    }
156}
157
158/// Backend for VercelBlob services.
159#[derive(Debug, Clone)]
160pub struct VercelBlobBackend {
161    core: Arc<VercelBlobCore>,
162}
163
164impl Access for VercelBlobBackend {
165    type Reader = HttpBody;
166    type Writer = VercelBlobWriters;
167    type Lister = oio::PageLister<VercelBlobLister>;
168    type Deleter = oio::OneShotDeleter<VercelBlobDeleter>;
169
170    fn info(&self) -> Arc<AccessorInfo> {
171        self.core.info.clone()
172    }
173
174    async fn stat(&self, path: &str, _args: OpStat) -> Result<RpStat> {
175        let resp = self.core.head(path).await?;
176
177        let status = resp.status();
178
179        match status {
180            StatusCode::OK => {
181                let bs = resp.into_body();
182
183                let resp: Blob =
184                    serde_json::from_reader(bs.reader()).map_err(new_json_deserialize_error)?;
185
186                parse_blob(&resp).map(RpStat::new)
187            }
188            _ => Err(parse_error(resp)),
189        }
190    }
191
192    async fn read(&self, path: &str, args: OpRead) -> Result<(RpRead, Self::Reader)> {
193        let resp = self.core.download(path, args.range(), &args).await?;
194
195        let status = resp.status();
196
197        match status {
198            StatusCode::OK | StatusCode::PARTIAL_CONTENT => {
199                Ok((RpRead::default(), resp.into_body()))
200            }
201            _ => {
202                let (part, mut body) = resp.into_parts();
203                let buf = body.to_buffer().await?;
204                Err(parse_error(Response::from_parts(part, buf)))
205            }
206        }
207    }
208
209    async fn write(&self, path: &str, args: OpWrite) -> Result<(RpWrite, Self::Writer)> {
210        let concurrent = args.concurrent();
211        let writer = VercelBlobWriter::new(self.core.clone(), args, path.to_string());
212
213        let w = oio::MultipartWriter::new(self.core.info.clone(), writer, concurrent);
214
215        Ok((RpWrite::default(), w))
216    }
217
218    async fn copy(&self, from: &str, to: &str, _args: OpCopy) -> Result<RpCopy> {
219        let resp = self.core.copy(from, to).await?;
220
221        let status = resp.status();
222
223        match status {
224            StatusCode::OK => Ok(RpCopy::default()),
225            _ => Err(parse_error(resp)),
226        }
227    }
228
229    async fn list(&self, path: &str, args: OpList) -> Result<(RpList, Self::Lister)> {
230        let l = VercelBlobLister::new(self.core.clone(), path, args.limit());
231        Ok((RpList::default(), oio::PageLister::new(l)))
232    }
233
234    async fn delete(&self) -> Result<(RpDelete, Self::Deleter)> {
235        Ok((
236            RpDelete::default(),
237            oio::OneShotDeleter::new(VercelBlobDeleter::new(self.core.clone())),
238        ))
239    }
240}
```
