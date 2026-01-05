# 

opendal/services/webhdfs/

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
18use core::fmt::Debug;
19use std::fmt::Formatter;
20use std::sync::Arc;
21
22use bytes::Buf;
23use http::Response;
24use http::StatusCode;
25use log::debug;
26use tokio::sync::OnceCell;
27
28use super::WEBHDFS_SCHEME;
29use super::core::WebhdfsCore;
30use super::delete::WebhdfsDeleter;
31use super::error::parse_error;
32use super::lister::WebhdfsLister;
33use super::message::BooleanResp;
34use super::message::FileStatusType;
35use super::message::FileStatusWrapper;
36use super::writer::WebhdfsWriter;
37use super::writer::WebhdfsWriters;
38use crate::raw::*;
39use crate::services::WebhdfsConfig;
40use crate::*;
41const WEBHDFS_DEFAULT_ENDPOINT: &str = "http://127.0.0.1:9870";
42
43/// [WebHDFS](https://hadoop.apache.org/docs/stable/hadoop-project-dist/hadoop-hdfs/WebHDFS.html)'s REST API support.
44#[doc = include_str!("docs.md")]
45#[derive(Default, Clone)]
46pub struct WebhdfsBuilder {
47    pub(super) config: WebhdfsConfig,
48}
49
50impl Debug for WebhdfsBuilder {
51    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
52        let mut d = f.debug_struct("WebhdfsBuilder");
53        d.field("config", &self.config);
54        d.finish_non_exhaustive()
55    }
56}
57
58impl WebhdfsBuilder {
59    /// Set the working directory of this backend
60    ///
61    /// All operations will happen under this root
62    ///
63    /// # Note
64    ///
65    /// The root will be automatically created if not exists.
66    pub fn root(mut self, root: &str) -> Self {
67        self.config.root = if root.is_empty() {
68            None
69        } else {
70            Some(root.to_string())
71        };
72
73        self
74    }
75
76    /// Set the remote address of this backend
77    /// default to `http://127.0.0.1:9870`
78    ///
79    /// Endpoints should be full uri, e.g.
80    ///
81    /// - `https://webhdfs.example.com:9870`
82    /// - `http://192.168.66.88:9870`
83    ///
84    /// If user inputs endpoint without scheme, we will
85    /// prepend `http://` to it.
86    pub fn endpoint(mut self, endpoint: &str) -> Self {
87        if !endpoint.is_empty() {
88            // trim tailing slash so we can accept `http://127.0.0.1:9870/`
89            self.config.endpoint = Some(endpoint.trim_end_matches('/').to_string());
90        }
91        self
92    }
93
94    /// Set the username of this backend,
95    /// used for authentication
96    pub fn user_name(mut self, user_name: &str) -> Self {
97        if !user_name.is_empty() {
98            self.config.user_name = Some(user_name.to_string());
99        }
100        self
101    }
102
103    /// Set the delegation token of this backend,
104    /// used for authentication
105    ///
106    /// # Note
107    /// The builder prefers using delegation token over username.
108    /// If both are set, delegation token will be used.
109    pub fn delegation(mut self, delegation: &str) -> Self {
110        if !delegation.is_empty() {
111            self.config.delegation = Some(delegation.to_string());
112        }
113        self
114    }
115
116    /// Disable batch listing
117    ///
118    /// # Note
119    ///
120    /// When listing a directory, the backend will default to use batch listing.
121    /// If disabled, the backend will list all files/directories in one request.
122    pub fn disable_list_batch(mut self) -> Self {
123        self.config.disable_list_batch = true;
124        self
125    }
126
127    /// Set temp dir for atomic write.
128    ///
129    /// # Notes
130    ///
131    /// If not set, write multi not support, eg: `.opendal_tmp/`.
132    pub fn atomic_write_dir(mut self, dir: &str) -> Self {
133        self.config.atomic_write_dir = if dir.is_empty() {
134            None
135        } else {
136            Some(String::from(dir))
137        };
138        self
139    }
140}
141
142impl Builder for WebhdfsBuilder {
143    type Config = WebhdfsConfig;
144
145    /// build the backend
146    ///
147    /// # Note
148    ///
149    /// when building backend, the built backend will check if the root directory
150    /// exits.
151    /// if the directory does not exit, the directory will be automatically created
152    fn build(self) -> Result<impl Access> {
153        debug!("start building backend: {self:?}");
154
155        let root = normalize_root(&self.config.root.unwrap_or_default());
156        debug!("backend use root {root}");
157
158        // check scheme
159        let endpoint = match self.config.endpoint {
160            Some(endpoint) => {
161                if endpoint.starts_with("http") {
162                    endpoint
163                } else {
164                    format!("http://{endpoint}")
165                }
166            }
167            None => WEBHDFS_DEFAULT_ENDPOINT.to_string(),
168        };
169        debug!("backend use endpoint {endpoint}");
170
171        let atomic_write_dir = self.config.atomic_write_dir;
172
173        let auth = self.config.delegation.map(|dt| format!("delegation={dt}"));
174
175        let info = AccessorInfo::default();
176        info.set_scheme(WEBHDFS_SCHEME)
177            .set_root(&root)
178            .set_native_capability(Capability {
179                stat: true,
180
181                read: true,
182
183                write: true,
184                write_can_append: true,
185                write_can_multi: atomic_write_dir.is_some(),
186
187                create_dir: true,
188                delete: true,
189
190                list: true,
191
192                shared: true,
193
194                ..Default::default()
195            });
196
197        let accessor_info = Arc::new(info);
198        let core = Arc::new(WebhdfsCore {
199            info: accessor_info,
200            root,
201            endpoint,
202            user_name: self.config.user_name,
203            auth,
204            root_checker: OnceCell::new(),
205            atomic_write_dir,
206            disable_list_batch: self.config.disable_list_batch,
207        });
208
209        Ok(WebhdfsBackend { core })
210    }
211}
212
213/// Backend for WebHDFS service
214#[derive(Debug, Clone)]
215pub struct WebhdfsBackend {
216    core: Arc<WebhdfsCore>,
217}
218
219impl WebhdfsBackend {
220    async fn check_root(&self) -> Result<()> {
221        let resp = self.core.webhdfs_get_file_status("/").await?;
222        match resp.status() {
223            StatusCode::OK => {
224                let bs = resp.into_body();
225
226                let file_status = serde_json::from_reader::<_, FileStatusWrapper>(bs.reader())
227                    .map_err(new_json_deserialize_error)?
228                    .file_status;
229
230                if file_status.ty == FileStatusType::File {
231                    return Err(Error::new(
232                        ErrorKind::ConfigInvalid,
233                        "root path must be dir",
234                    ));
235                }
236            }
237            StatusCode::NOT_FOUND => {
238                self.create_dir("/", OpCreateDir::new()).await?;
239            }
240            _ => return Err(parse_error(resp)),
241        }
242        Ok(())
243    }
244}
245
246impl Access for WebhdfsBackend {
247    type Reader = HttpBody;
248    type Writer = WebhdfsWriters;
249    type Lister = oio::PageLister<WebhdfsLister>;
250    type Deleter = oio::OneShotDeleter<WebhdfsDeleter>;
251
252    fn info(&self) -> Arc<AccessorInfo> {
253        self.core.info.clone()
254    }
255
256    /// Create a file or directory
257    async fn create_dir(&self, path: &str, _: OpCreateDir) -> Result<RpCreateDir> {
258        let resp = self.core.webhdfs_create_dir(path).await?;
259
260        let status = resp.status();
261        // WebHDFS's has a two-step create/append to prevent clients to send out
262        // data before creating it.
263        // According to the redirect policy of `reqwest` HTTP Client we are using,
264        // the redirection should be done automatically.
265        match status {
266            StatusCode::CREATED | StatusCode::OK => {
267                let bs = resp.into_body();
268
269                let resp = serde_json::from_reader::<_, BooleanResp>(bs.reader())
270                    .map_err(new_json_deserialize_error)?;
271
272                if resp.boolean {
273                    Ok(RpCreateDir::default())
274                } else {
275                    Err(Error::new(
276                        ErrorKind::Unexpected,
277                        "webhdfs create dir failed",
278                    ))
279                }
280            }
281            _ => Err(parse_error(resp)),
282        }
283    }
284
285    async fn stat(&self, path: &str, _: OpStat) -> Result<RpStat> {
286        // if root exists and is a directory, stat will be ok
287        self.core
288            .root_checker
289            .get_or_try_init(|| async { self.check_root().await })
290            .await?;
291
292        let resp = self.core.webhdfs_get_file_status(path).await?;
293        let status = resp.status();
294        match status {
295            StatusCode::OK => {
296                let bs = resp.into_body();
297
298                let file_status = serde_json::from_reader::<_, FileStatusWrapper>(bs.reader())
299                    .map_err(new_json_deserialize_error)?
300                    .file_status;
301
302                let meta = match file_status.ty {
303                    FileStatusType::Directory => Metadata::new(EntryMode::DIR),
304                    FileStatusType::File => Metadata::new(EntryMode::FILE)
305                        .with_content_length(file_status.length)
306                        .with_last_modified(Timestamp::from_millisecond(
307                            file_status.modification_time,
308                        )?),
309                };
310
311                Ok(RpStat::new(meta))
312            }
313
314            _ => Err(parse_error(resp)),
315        }
316    }
317
318    async fn read(&self, path: &str, args: OpRead) -> Result<(RpRead, Self::Reader)> {
319        let resp = self.core.webhdfs_read_file(path, args.range()).await?;
320
321        let status = resp.status();
322        match status {
323            StatusCode::OK | StatusCode::PARTIAL_CONTENT => {
324                Ok((RpRead::default(), resp.into_body()))
325            }
326            _ => {
327                let (part, mut body) = resp.into_parts();
328                let buf = body.to_buffer().await?;
329                Err(parse_error(Response::from_parts(part, buf)))
330            }
331        }
332    }
333
334    async fn write(&self, path: &str, args: OpWrite) -> Result<(RpWrite, Self::Writer)> {
335        let w = WebhdfsWriter::new(self.core.clone(), args.clone(), path.to_string());
336
337        let w = if args.append() {
338            WebhdfsWriters::Two(oio::AppendWriter::new(w))
339        } else {
340            WebhdfsWriters::One(oio::BlockWriter::new(
341                self.info().clone(),
342                w,
343                args.concurrent(),
344            ))
345        };
346
347        Ok((RpWrite::default(), w))
348    }
349
350    async fn delete(&self) -> Result<(RpDelete, Self::Deleter)> {
351        Ok((
352            RpDelete::default(),
353            oio::OneShotDeleter::new(WebhdfsDeleter::new(self.core.clone())),
354        ))
355    }
356
357    async fn list(&self, path: &str, args: OpList) -> Result<(RpList, Self::Lister)> {
358        if args.recursive() {
359            return Err(Error::new(
360                ErrorKind::Unsupported,
361                "WebHDFS doesn't support list with recursive",
362            ));
363        }
364
365        let path = path.trim_end_matches('/');
366        let l = WebhdfsLister::new(self.core.clone(), path);
367        Ok((RpList::default(), oio::PageLister::new(l)))
368    }
369}
```
