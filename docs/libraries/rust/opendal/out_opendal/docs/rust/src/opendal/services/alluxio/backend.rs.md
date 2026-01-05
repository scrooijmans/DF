# 

opendal/services/alluxio/

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
22use http::Response;
23use log::debug;
24
25use super::ALLUXIO_SCHEME;
26use super::core::AlluxioCore;
27use super::delete::AlluxioDeleter;
28use super::error::parse_error;
29use super::lister::AlluxioLister;
30use super::writer::AlluxioWriter;
31use super::writer::AlluxioWriters;
32use crate::raw::*;
33use crate::services::AlluxioConfig;
34use crate::*;
35
36/// [Alluxio](https://www.alluxio.io/) services support.
37#[doc = include_str!("docs.md")]
38#[derive(Default)]
39pub struct AlluxioBuilder {
40    pub(super) config: AlluxioConfig,
41
42    #[deprecated(since = "0.53.0", note = "Use `Operator::update_http_client` instead")]
43    pub(super) http_client: Option<HttpClient>,
44}
45
46impl Debug for AlluxioBuilder {
47    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
48        let mut d = f.debug_struct("AlluxioBuilder");
49
50        d.field("config", &self.config);
51        d.finish_non_exhaustive()
52    }
53}
54
55impl AlluxioBuilder {
56    /// Set root of this backend.
57    ///
58    /// All operations will happen under this root.
59    pub fn root(mut self, root: &str) -> Self {
60        self.config.root = if root.is_empty() {
61            None
62        } else {
63            Some(root.to_string())
64        };
65
66        self
67    }
68
69    /// endpoint of this backend.
70    ///
71    /// Endpoint must be full uri, mostly like `http://127.0.0.1:39999`.
72    pub fn endpoint(mut self, endpoint: &str) -> Self {
73        if !endpoint.is_empty() {
74            // Trim trailing `/` so that we can accept `http://127.0.0.1:39999/`
75            self.config.endpoint = Some(endpoint.trim_end_matches('/').to_string())
76        }
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
95impl Builder for AlluxioBuilder {
96    type Config = AlluxioConfig;
97
98    /// Builds the backend and returns the result of AlluxioBackend.
99    fn build(self) -> Result<impl Access> {
100        debug!("backend build started: {:?}", &self);
101
102        let root = normalize_root(&self.config.root.clone().unwrap_or_default());
103        debug!("backend use root {}", &root);
104
105        let endpoint = match &self.config.endpoint {
106            Some(endpoint) => Ok(endpoint.clone()),
107            None => Err(Error::new(ErrorKind::ConfigInvalid, "endpoint is empty")
108                .with_operation("Builder::build")
109                .with_context("service", Scheme::Alluxio)),
110        }?;
111        debug!("backend use endpoint {}", &endpoint);
112
113        Ok(AlluxioBackend {
114            core: Arc::new(AlluxioCore {
115                info: {
116                    let am = AccessorInfo::default();
117                    am.set_scheme(ALLUXIO_SCHEME)
118                        .set_root(&root)
119                        .set_native_capability(Capability {
120                            stat: true,
121
122                            // FIXME:
123                            //
124                            // alluxio's read support is not implemented correctly
125                            // We need to refactor by use [page_read](https://github.com/Alluxio/alluxio-py/blob/main/alluxio/const.py#L18)
126                            read: false,
127
128                            write: true,
129                            write_can_multi: true,
130
131                            create_dir: true,
132                            delete: true,
133
134                            list: true,
135
136                            shared: true,
137
138                            ..Default::default()
139                        });
140
141                    // allow deprecated api here for compatibility
142                    #[allow(deprecated)]
143                    if let Some(client) = self.http_client {
144                        am.update_http_client(|_| client);
145                    }
146
147                    am.into()
148                },
149                root,
150                endpoint,
151            }),
152        })
153    }
154}
155
156#[derive(Debug, Clone)]
157pub struct AlluxioBackend {
158    core: Arc<AlluxioCore>,
159}
160
161impl Access for AlluxioBackend {
162    type Reader = HttpBody;
163    type Writer = AlluxioWriters;
164    type Lister = oio::PageLister<AlluxioLister>;
165    type Deleter = oio::OneShotDeleter<AlluxioDeleter>;
166
167    fn info(&self) -> Arc<AccessorInfo> {
168        self.core.info.clone()
169    }
170
171    async fn create_dir(&self, path: &str, _: OpCreateDir) -> Result<RpCreateDir> {
172        self.core.create_dir(path).await?;
173        Ok(RpCreateDir::default())
174    }
175
176    async fn stat(&self, path: &str, _: OpStat) -> Result<RpStat> {
177        let file_info = self.core.get_status(path).await?;
178
179        Ok(RpStat::new(file_info.try_into()?))
180    }
181
182    async fn read(&self, path: &str, args: OpRead) -> Result<(RpRead, Self::Reader)> {
183        let stream_id = self.core.open_file(path).await?;
184
185        let resp = self.core.read(stream_id, args.range()).await?;
186        if !resp.status().is_success() {
187            let (part, mut body) = resp.into_parts();
188            let buf = body.to_buffer().await?;
189            return Err(parse_error(Response::from_parts(part, buf)));
190        }
191        Ok((RpRead::new(), resp.into_body()))
192    }
193
194    async fn write(&self, path: &str, args: OpWrite) -> Result<(RpWrite, Self::Writer)> {
195        let w = AlluxioWriter::new(self.core.clone(), args.clone(), path.to_string());
196
197        Ok((RpWrite::default(), w))
198    }
199
200    async fn delete(&self) -> Result<(RpDelete, Self::Deleter)> {
201        Ok((
202            RpDelete::default(),
203            oio::OneShotDeleter::new(AlluxioDeleter::new(self.core.clone())),
204        ))
205    }
206
207    async fn list(&self, path: &str, _args: OpList) -> Result<(RpList, Self::Lister)> {
208        let l = AlluxioLister::new(self.core.clone(), path);
209        Ok((RpList::default(), oio::PageLister::new(l)))
210    }
211
212    async fn rename(&self, from: &str, to: &str, _: OpRename) -> Result<RpRename> {
213        self.core.rename(from, to).await?;
214
215        Ok(RpRename::default())
216    }
217}
218
219#[cfg(test)]
220mod test {
221    use std::collections::HashMap;
222
223    use super::*;
224
225    #[test]
226    fn test_builder_from_map() {
227        let mut map = HashMap::new();
228        map.insert("root".to_string(), "/".to_string());
229        map.insert("endpoint".to_string(), "http://127.0.0.1:39999".to_string());
230
231        let builder = AlluxioConfig::from_iter(map).unwrap();
232
233        assert_eq!(builder.root, Some("/".to_string()));
234        assert_eq!(builder.endpoint, Some("http://127.0.0.1:39999".to_string()));
235    }
236
237    #[test]
238    fn test_builder_build() {
239        let builder = AlluxioBuilder::default()
240            .root("/root")
241            .endpoint("http://127.0.0.1:39999")
242            .build();
243
244        assert!(builder.is_ok());
245    }
246}
```
