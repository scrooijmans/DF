# 

opendal/services/upyun/

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
23use http::StatusCode;
24use log::debug;
25
26use super::UPYUN_SCHEME;
27use super::core::*;
28use super::delete::UpyunDeleter;
29use super::error::parse_error;
30use super::lister::UpyunLister;
31use super::writer::UpyunWriter;
32use super::writer::UpyunWriters;
33use crate::raw::*;
34use crate::services::UpyunConfig;
35use crate::*;
36
37/// [upyun](https://www.upyun.com/products/file-storage) services support.
38#[doc = include_str!("docs.md")]
39#[derive(Default)]
40pub struct UpyunBuilder {
41    pub(super) config: UpyunConfig,
42
43    #[deprecated(since = "0.53.0", note = "Use `Operator::update_http_client` instead")]
44    pub(super) http_client: Option<HttpClient>,
45}
46
47impl Debug for UpyunBuilder {
48    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
49        let mut d = f.debug_struct("UpyunBuilder");
50
51        d.field("config", &self.config);
52        d.finish_non_exhaustive()
53    }
54}
55
56impl UpyunBuilder {
57    /// Set root of this backend.
58    ///
59    /// All operations will happen under this root.
60    pub fn root(mut self, root: &str) -> Self {
61        self.config.root = if root.is_empty() {
62            None
63        } else {
64            Some(root.to_string())
65        };
66
67        self
68    }
69
70    /// bucket of this backend.
71    ///
72    /// It is required. e.g. `test`
73    pub fn bucket(mut self, bucket: &str) -> Self {
74        self.config.bucket = bucket.to_string();
75
76        self
77    }
78
79    /// operator of this backend.
80    ///
81    /// It is required. e.g. `test`
82    pub fn operator(mut self, operator: &str) -> Self {
83        self.config.operator = if operator.is_empty() {
84            None
85        } else {
86            Some(operator.to_string())
87        };
88
89        self
90    }
91
92    /// password of this backend.
93    ///
94    /// It is required. e.g. `asecret`
95    pub fn password(mut self, password: &str) -> Self {
96        self.config.password = if password.is_empty() {
97            None
98        } else {
99            Some(password.to_string())
100        };
101
102        self
103    }
104
105    /// Specify the http client that used by this service.
106    ///
107    /// # Notes
108    ///
109    /// This API is part of OpenDAL's Raw API. `HttpClient` could be changed
110    /// during minor updates.
111    #[deprecated(since = "0.53.0", note = "Use `Operator::update_http_client` instead")]
112    #[allow(deprecated)]
113    pub fn http_client(mut self, client: HttpClient) -> Self {
114        self.http_client = Some(client);
115        self
116    }
117}
118
119impl Builder for UpyunBuilder {
120    type Config = UpyunConfig;
121
122    /// Builds the backend and returns the result of UpyunBackend.
123    fn build(self) -> Result<impl Access> {
124        debug!("backend build started: {:?}", &self);
125
126        let root = normalize_root(&self.config.root.clone().unwrap_or_default());
127        debug!("backend use root {}", &root);
128
129        // Handle bucket.
130        if self.config.bucket.is_empty() {
131            return Err(Error::new(ErrorKind::ConfigInvalid, "bucket is empty")
132                .with_operation("Builder::build")
133                .with_context("service", Scheme::Upyun));
134        }
135
136        debug!("backend use bucket {}", &self.config.bucket);
137
138        let operator = match &self.config.operator {
139            Some(operator) => Ok(operator.clone()),
140            None => Err(Error::new(ErrorKind::ConfigInvalid, "operator is empty")
141                .with_operation("Builder::build")
142                .with_context("service", Scheme::Upyun)),
143        }?;
144
145        let password = match &self.config.password {
146            Some(password) => Ok(password.clone()),
147            None => Err(Error::new(ErrorKind::ConfigInvalid, "password is empty")
148                .with_operation("Builder::build")
149                .with_context("service", Scheme::Upyun)),
150        }?;
151
152        let signer = UpyunSigner {
153            operator: operator.clone(),
154            password: password.clone(),
155        };
156
157        Ok(UpyunBackend {
158            core: Arc::new(UpyunCore {
159                info: {
160                    let am = AccessorInfo::default();
161                    am.set_scheme(UPYUN_SCHEME)
162                        .set_root(&root)
163                        .set_native_capability(Capability {
164                            stat: true,
165
166                            create_dir: true,
167
168                            read: true,
169
170                            write: true,
171                            write_can_empty: true,
172                            write_can_multi: true,
173                            write_with_cache_control: true,
174                            write_with_content_type: true,
175
176                            // https://help.upyun.com/knowledge-base/rest_api/#e5b9b6e8a18ce5bc8fe696ade782b9e7bbade4bca0
177                            write_multi_min_size: Some(1024 * 1024),
178                            write_multi_max_size: Some(50 * 1024 * 1024),
179
180                            delete: true,
181                            rename: true,
182                            copy: true,
183
184                            list: true,
185                            list_with_limit: true,
186
187                            shared: true,
188
189                            ..Default::default()
190                        });
191
192                    // allow deprecated api here for compatibility
193                    #[allow(deprecated)]
194                    if let Some(client) = self.http_client {
195                        am.update_http_client(|_| client);
196                    }
197
198                    am.into()
199                },
200                root,
201                operator,
202                bucket: self.config.bucket.clone(),
203                signer,
204            }),
205        })
206    }
207}
208
209/// Backend for upyun services.
210#[derive(Debug, Clone)]
211pub struct UpyunBackend {
212    core: Arc<UpyunCore>,
213}
214
215impl Access for UpyunBackend {
216    type Reader = HttpBody;
217    type Writer = UpyunWriters;
218    type Lister = oio::PageLister<UpyunLister>;
219    type Deleter = oio::OneShotDeleter<UpyunDeleter>;
220
221    fn info(&self) -> Arc<AccessorInfo> {
222        self.core.info.clone()
223    }
224
225    async fn create_dir(&self, path: &str, _: OpCreateDir) -> Result<RpCreateDir> {
226        let resp = self.core.create_dir(path).await?;
227
228        let status = resp.status();
229
230        match status {
231            StatusCode::OK => Ok(RpCreateDir::default()),
232            _ => Err(parse_error(resp)),
233        }
234    }
235
236    async fn stat(&self, path: &str, _args: OpStat) -> Result<RpStat> {
237        let resp = self.core.info(path).await?;
238
239        let status = resp.status();
240
241        match status {
242            StatusCode::OK => parse_info(resp.headers()).map(RpStat::new),
243            _ => Err(parse_error(resp)),
244        }
245    }
246
247    async fn read(&self, path: &str, args: OpRead) -> Result<(RpRead, Self::Reader)> {
248        let resp = self.core.download_file(path, args.range()).await?;
249
250        let status = resp.status();
251
252        match status {
253            StatusCode::OK | StatusCode::PARTIAL_CONTENT => {
254                Ok((RpRead::default(), resp.into_body()))
255            }
256            _ => {
257                let (part, mut body) = resp.into_parts();
258                let buf = body.to_buffer().await?;
259                Err(parse_error(Response::from_parts(part, buf)))
260            }
261        }
262    }
263
264    async fn write(&self, path: &str, args: OpWrite) -> Result<(RpWrite, Self::Writer)> {
265        let concurrent = args.concurrent();
266        let writer = UpyunWriter::new(self.core.clone(), args, path.to_string());
267
268        let w = oio::MultipartWriter::new(self.core.info.clone(), writer, concurrent);
269
270        Ok((RpWrite::default(), w))
271    }
272
273    async fn delete(&self) -> Result<(RpDelete, Self::Deleter)> {
274        Ok((
275            RpDelete::default(),
276            oio::OneShotDeleter::new(UpyunDeleter::new(self.core.clone())),
277        ))
278    }
279
280    async fn list(&self, path: &str, args: OpList) -> Result<(RpList, Self::Lister)> {
281        let l = UpyunLister::new(self.core.clone(), path, args.limit());
282        Ok((RpList::default(), oio::PageLister::new(l)))
283    }
284
285    async fn copy(&self, from: &str, to: &str, _args: OpCopy) -> Result<RpCopy> {
286        let resp = self.core.copy(from, to).await?;
287
288        let status = resp.status();
289
290        match status {
291            StatusCode::OK => Ok(RpCopy::default()),
292            _ => Err(parse_error(resp)),
293        }
294    }
295
296    async fn rename(&self, from: &str, to: &str, _args: OpRename) -> Result<RpRename> {
297        let resp = self.core.move_object(from, to).await?;
298
299        let status = resp.status();
300
301        match status {
302            StatusCode::OK => Ok(RpRename::default()),
303            _ => Err(parse_error(resp)),
304        }
305    }
306}
```
