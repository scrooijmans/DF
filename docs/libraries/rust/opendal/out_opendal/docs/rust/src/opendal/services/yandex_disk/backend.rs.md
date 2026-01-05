# 

opendal/services/yandex_disk/

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
27use super::YANDEX_DISK_SCHEME;
28use super::core::*;
29use super::delete::YandexDiskDeleter;
30use super::error::parse_error;
31use super::lister::YandexDiskLister;
32use super::writer::YandexDiskWriter;
33use super::writer::YandexDiskWriters;
34use crate::raw::*;
35use crate::services::YandexDiskConfig;
36use crate::*;
37
38/// [YandexDisk](https://360.yandex.com/disk/) services support.
39#[doc = include_str!("docs.md")]
40#[derive(Default)]
41pub struct YandexDiskBuilder {
42    pub(super) config: YandexDiskConfig,
43
44    #[deprecated(since = "0.53.0", note = "Use `Operator::update_http_client` instead")]
45    pub(super) http_client: Option<HttpClient>,
46}
47
48impl Debug for YandexDiskBuilder {
49    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
50        let mut d = f.debug_struct("YandexDiskBuilder");
51
52        d.field("config", &self.config);
53        d.finish_non_exhaustive()
54    }
55}
56
57impl YandexDiskBuilder {
58    /// Set root of this backend.
59    ///
60    /// All operations will happen under this root.
61    pub fn root(mut self, root: &str) -> Self {
62        self.config.root = if root.is_empty() {
63            None
64        } else {
65            Some(root.to_string())
66        };
67
68        self
69    }
70
71    /// yandex disk oauth access_token.
72    /// The valid token will looks like `y0_XXXXXXqihqIWAADLWwAAAAD3IXXXXXX0gtVeSPeIKM0oITMGhXXXXXX`.
73    /// We can fetch the debug token from <https://yandex.com/dev/disk/poligon>.
74    /// To use it in production, please register an app at <https://oauth.yandex.com> instead.
75    pub fn access_token(mut self, access_token: &str) -> Self {
76        self.config.access_token = access_token.to_string();
77
78        self
79    }
80
81    /// Specify the http client that used by this service.
82    ///
83    /// # Notes
84    ///
85    /// This API is part of OpenDAL's Raw API. `HttpClient` could be changed
86    /// during minor updates.
87    #[deprecated(since = "0.53.0", note = "Use `Operator::update_http_client` instead")]
88    #[allow(deprecated)]
89    pub fn http_client(mut self, client: HttpClient) -> Self {
90        self.http_client = Some(client);
91        self
92    }
93}
94
95impl Builder for YandexDiskBuilder {
96    type Config = YandexDiskConfig;
97
98    /// Builds the backend and returns the result of YandexDiskBackend.
99    fn build(self) -> Result<impl Access> {
100        debug!("backend build started: {:?}", &self);
101
102        let root = normalize_root(&self.config.root.clone().unwrap_or_default());
103        debug!("backend use root {}", &root);
104
105        // Handle oauth access_token.
106        if self.config.access_token.is_empty() {
107            return Err(
108                Error::new(ErrorKind::ConfigInvalid, "access_token is empty")
109                    .with_operation("Builder::build")
110                    .with_context("service", Scheme::YandexDisk),
111            );
112        }
113
114        Ok(YandexDiskBackend {
115            core: Arc::new(YandexDiskCore {
116                info: {
117                    let am = AccessorInfo::default();
118                    am.set_scheme(YANDEX_DISK_SCHEME)
119                        .set_root(&root)
120                        .set_native_capability(Capability {
121                            stat: true,
122
123                            create_dir: true,
124
125                            read: true,
126
127                            write: true,
128                            write_can_empty: true,
129
130                            delete: true,
131                            rename: true,
132                            copy: true,
133
134                            list: true,
135                            list_with_limit: true,
136
137                            shared: true,
138
139                            ..Default::default()
140                        });
141
142                    // allow deprecated api here for compatibility
143                    #[allow(deprecated)]
144                    if let Some(client) = self.http_client {
145                        am.update_http_client(|_| client);
146                    }
147
148                    am.into()
149                },
150                root,
151                access_token: self.config.access_token.clone(),
152            }),
153        })
154    }
155}
156
157/// Backend for YandexDisk services.
158#[derive(Debug, Clone)]
159pub struct YandexDiskBackend {
160    core: Arc<YandexDiskCore>,
161}
162
163impl Access for YandexDiskBackend {
164    type Reader = HttpBody;
165    type Writer = YandexDiskWriters;
166    type Lister = oio::PageLister<YandexDiskLister>;
167    type Deleter = oio::OneShotDeleter<YandexDiskDeleter>;
168
169    fn info(&self) -> Arc<AccessorInfo> {
170        self.core.info.clone()
171    }
172
173    async fn create_dir(&self, path: &str, _: OpCreateDir) -> Result<RpCreateDir> {
174        self.core.ensure_dir_exists(path).await?;
175
176        Ok(RpCreateDir::default())
177    }
178
179    async fn rename(&self, from: &str, to: &str, _args: OpRename) -> Result<RpRename> {
180        self.core.ensure_dir_exists(to).await?;
181
182        let resp = self.core.move_object(from, to).await?;
183
184        let status = resp.status();
185
186        match status {
187            StatusCode::OK | StatusCode::CREATED => Ok(RpRename::default()),
188            _ => Err(parse_error(resp)),
189        }
190    }
191
192    async fn copy(&self, from: &str, to: &str, _args: OpCopy) -> Result<RpCopy> {
193        self.core.ensure_dir_exists(to).await?;
194
195        let resp = self.core.copy(from, to).await?;
196
197        let status = resp.status();
198
199        match status {
200            StatusCode::OK | StatusCode::CREATED => Ok(RpCopy::default()),
201            _ => Err(parse_error(resp)),
202        }
203    }
204
205    async fn read(&self, path: &str, args: OpRead) -> Result<(RpRead, Self::Reader)> {
206        let resp = self.core.download(path, args.range()).await?;
207
208        let status = resp.status();
209        match status {
210            StatusCode::OK | StatusCode::PARTIAL_CONTENT => Ok((RpRead::new(), resp.into_body())),
211            _ => {
212                let (part, mut body) = resp.into_parts();
213                let buf = body.to_buffer().await?;
214                Err(parse_error(Response::from_parts(part, buf)))
215            }
216        }
217    }
218
219    async fn stat(&self, path: &str, _args: OpStat) -> Result<RpStat> {
220        let resp = self.core.metainformation(path, None, None).await?;
221
222        let status = resp.status();
223
224        match status {
225            StatusCode::OK => {
226                let bs = resp.into_body();
227
228                let mf: MetainformationResponse =
229                    serde_json::from_reader(bs.reader()).map_err(new_json_deserialize_error)?;
230
231                parse_info(mf).map(RpStat::new)
232            }
233            _ => Err(parse_error(resp)),
234        }
235    }
236
237    async fn write(&self, path: &str, _args: OpWrite) -> Result<(RpWrite, Self::Writer)> {
238        let writer = YandexDiskWriter::new(self.core.clone(), path.to_string());
239
240        let w = oio::OneShotWriter::new(writer);
241
242        Ok((RpWrite::default(), w))
243    }
244
245    async fn delete(&self) -> Result<(RpDelete, Self::Deleter)> {
246        Ok((
247            RpDelete::default(),
248            oio::OneShotDeleter::new(YandexDiskDeleter::new(self.core.clone())),
249        ))
250    }
251
252    async fn list(&self, path: &str, args: OpList) -> Result<(RpList, Self::Lister)> {
253        let l = YandexDiskLister::new(self.core.clone(), path, args.limit());
254        Ok((RpList::default(), oio::PageLister::new(l)))
255    }
256}
```
