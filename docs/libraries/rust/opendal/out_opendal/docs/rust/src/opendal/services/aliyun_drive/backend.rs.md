# 

opendal/services/aliyun_drive/

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
26use tokio::sync::Mutex;
27
28use super::ALIYUN_DRIVE_SCHEME;
29use super::core::*;
30use super::delete::AliyunDriveDeleter;
31use super::error::parse_error;
32use super::lister::AliyunDriveLister;
33use super::lister::AliyunDriveParent;
34use super::writer::AliyunDriveWriter;
35use crate::raw::*;
36use crate::services::AliyunDriveConfig;
37use crate::*;
38
39#[doc = include_str!("docs.md")]
40#[derive(Default)]
41pub struct AliyunDriveBuilder {
42    pub(super) config: AliyunDriveConfig,
43
44    #[deprecated(since = "0.53.0", note = "Use `Operator::update_http_client` instead")]
45    pub(super) http_client: Option<HttpClient>,
46}
47
48impl Debug for AliyunDriveBuilder {
49    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
50        let mut d = f.debug_struct("AliyunDriveBuilder");
51
52        d.field("config", &self.config);
53        d.finish_non_exhaustive()
54    }
55}
56
57impl AliyunDriveBuilder {
58    /// Set the root of this backend.
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
71    /// Set access_token of this backend.
72    pub fn access_token(mut self, access_token: &str) -> Self {
73        self.config.access_token = Some(access_token.to_string());
74
75        self
76    }
77
78    /// Set client_id of this backend.
79    pub fn client_id(mut self, client_id: &str) -> Self {
80        self.config.client_id = Some(client_id.to_string());
81
82        self
83    }
84
85    /// Set client_secret of this backend.
86    pub fn client_secret(mut self, client_secret: &str) -> Self {
87        self.config.client_secret = Some(client_secret.to_string());
88
89        self
90    }
91
92    /// Set refresh_token of this backend.
93    pub fn refresh_token(mut self, refresh_token: &str) -> Self {
94        self.config.refresh_token = Some(refresh_token.to_string());
95
96        self
97    }
98
99    /// Set drive_type of this backend.
100    pub fn drive_type(mut self, drive_type: &str) -> Self {
101        self.config.drive_type = drive_type.to_string();
102
103        self
104    }
105
106    /// Specify the http client that used by this service.
107    ///
108    /// # Notes
109    ///
110    /// This API is part of OpenDAL's Raw API. `HttpClient` could be changed
111    /// during minor updates.
112    #[deprecated(since = "0.53.0", note = "Use `Operator::update_http_client` instead")]
113    #[allow(deprecated)]
114    pub fn http_client(mut self, client: HttpClient) -> Self {
115        self.http_client = Some(client);
116        self
117    }
118}
119
120impl Builder for AliyunDriveBuilder {
121    type Config = AliyunDriveConfig;
122
123    fn build(self) -> Result<impl Access> {
124        debug!("backend build started: {:?}", &self);
125
126        let root = normalize_root(&self.config.root.clone().unwrap_or_default());
127        debug!("backend use root {}", &root);
128
129        let sign = match self.config.access_token.clone() {
130            Some(access_token) if !access_token.is_empty() => {
131                AliyunDriveSign::Access(access_token)
132            }
133            _ => match (
134                self.config.client_id.clone(),
135                self.config.client_secret.clone(),
136                self.config.refresh_token.clone(),
137            ) {
138                (Some(client_id), Some(client_secret), Some(refresh_token)) if
139                !client_id.is_empty() && !client_secret.is_empty() && !refresh_token.is_empty() => {
140                    AliyunDriveSign::Refresh(client_id, client_secret, refresh_token, None, 0)
141                }
142                _ => return Err(Error::new(
143                    ErrorKind::ConfigInvalid,
144                    "access_token and a set of client_id, client_secret, and refresh_token are both missing.")
145                    .with_operation("Builder::build")
146                    .with_context("service", Scheme::AliyunDrive)),
147            },
148        };
149
150        let drive_type = match self.config.drive_type.as_str() {
151            "" | "default" => DriveType::Default,
152            "resource" => DriveType::Resource,
153            "backup" => DriveType::Backup,
154            _ => {
155                return Err(Error::new(
156                    ErrorKind::ConfigInvalid,
157                    "drive_type is invalid.",
158                ));
159            }
160        };
161        debug!("backend use drive_type {drive_type:?}");
162
163        Ok(AliyunDriveBackend {
164            core: Arc::new(AliyunDriveCore {
165                info: {
166                    let am = AccessorInfo::default();
167                    am.set_scheme(ALIYUN_DRIVE_SCHEME)
168                        .set_root(&root)
169                        .set_native_capability(Capability {
170                            stat: true,
171                            create_dir: true,
172                            read: true,
173                            write: true,
174                            write_can_multi: true,
175                            // The min multipart size of AliyunDrive is 100 KiB.
176                            write_multi_min_size: Some(100 * 1024),
177                            // The max multipart size of AliyunDrive is 5 GiB.
178                            write_multi_max_size: if cfg!(target_pointer_width = "64") {
179                                Some(5 * 1024 * 1024 * 1024)
180                            } else {
181                                Some(usize::MAX)
182                            },
183                            delete: true,
184                            copy: true,
185                            rename: true,
186                            list: true,
187                            list_with_limit: true,
188                            shared: true,
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
200                endpoint: "https://openapi.alipan.com".to_string(),
201                root,
202                drive_type,
203                signer: Arc::new(Mutex::new(AliyunDriveSigner {
204                    drive_id: None,
205                    sign,
206                })),
207                dir_lock: Arc::new(Mutex::new(())),
208            }),
209        })
210    }
211}
212
213#[derive(Clone, Debug)]
214pub struct AliyunDriveBackend {
215    core: Arc<AliyunDriveCore>,
216}
217
218impl Access for AliyunDriveBackend {
219    type Reader = HttpBody;
220    type Writer = AliyunDriveWriter;
221    type Lister = oio::PageLister<AliyunDriveLister>;
222    type Deleter = oio::OneShotDeleter<AliyunDriveDeleter>;
223
224    fn info(&self) -> Arc<AccessorInfo> {
225        self.core.info.clone()
226    }
227
228    async fn create_dir(&self, path: &str, _args: OpCreateDir) -> Result<RpCreateDir> {
229        self.core.ensure_dir_exists(path).await?;
230
231        Ok(RpCreateDir::default())
232    }
233
234    async fn rename(&self, from: &str, to: &str, _args: OpRename) -> Result<RpRename> {
235        if from == to {
236            return Ok(RpRename::default());
237        }
238        let res = self.core.get_by_path(from).await?;
239        let file: AliyunDriveFile =
240            serde_json::from_reader(res.reader()).map_err(new_json_serialize_error)?;
241        // rename can overwrite.
242        match self.core.get_by_path(to).await {
243            Err(err) if err.kind() == ErrorKind::NotFound => {}
244            Err(err) => return Err(err),
245            Ok(res) => {
246                let file: AliyunDriveFile =
247                    serde_json::from_reader(res.reader()).map_err(new_json_serialize_error)?;
248                self.core.delete_path(&file.file_id).await?;
249            }
250        };
251
252        let parent_file_id = self.core.ensure_dir_exists(get_parent(to)).await?;
253        self.core.move_path(&file.file_id, &parent_file_id).await?;
254
255        let from_name = get_basename(from);
256        let to_name = get_basename(to);
257
258        if from_name != to_name {
259            self.core.update_path(&file.file_id, to_name).await?;
260        }
261
262        Ok(RpRename::default())
263    }
264
265    async fn copy(&self, from: &str, to: &str, _args: OpCopy) -> Result<RpCopy> {
266        if from == to {
267            return Ok(RpCopy::default());
268        }
269        let res = self.core.get_by_path(from).await?;
270        let file: AliyunDriveFile =
271            serde_json::from_reader(res.reader()).map_err(new_json_serialize_error)?;
272        // copy can overwrite.
273        match self.core.get_by_path(to).await {
274            Err(err) if err.kind() == ErrorKind::NotFound => {}
275            Err(err) => return Err(err),
276            Ok(res) => {
277                let file: AliyunDriveFile =
278                    serde_json::from_reader(res.reader()).map_err(new_json_serialize_error)?;
279                self.core.delete_path(&file.file_id).await?;
280            }
281        };
282        // there is no direct copy in AliyunDrive.
283        // so we need to copy the path first and then rename it.
284        let parent_path = get_parent(to);
285        let parent_file_id = self.core.ensure_dir_exists(parent_path).await?;
286
287        // if from and to are going to be placed in the same folder,
288        // copy_path will fail as we cannot change the name during this action.
289        // it has to be auto renamed.
290        let auto_rename = file.parent_file_id == parent_file_id;
291        let res = self
292            .core
293            .copy_path(&file.file_id, &parent_file_id, auto_rename)
294            .await?;
295        let file: CopyResponse =
296            serde_json::from_reader(res.reader()).map_err(new_json_serialize_error)?;
297        let file_id = file.file_id;
298
299        let from_name = get_basename(from);
300        let to_name = get_basename(to);
301
302        if from_name != to_name {
303            self.core.update_path(&file_id, to_name).await?;
304        }
305
306        Ok(RpCopy::default())
307    }
308
309    async fn stat(&self, path: &str, _args: OpStat) -> Result<RpStat> {
310        let res = self.core.get_by_path(path).await?;
311        let file: AliyunDriveFile =
312            serde_json::from_reader(res.reader()).map_err(new_json_serialize_error)?;
313
314        if file.path_type == "folder" {
315            let meta = Metadata::new(EntryMode::DIR).with_last_modified(
316                file.updated_at.parse::<Timestamp>().map_err(|e| {
317                    Error::new(ErrorKind::Unexpected, "parse last modified time").set_source(e)
318                })?,
319            );
320
321            return Ok(RpStat::new(meta));
322        }
323
324        let mut meta = Metadata::new(EntryMode::FILE).with_last_modified(
325            file.updated_at.parse::<Timestamp>().map_err(|e| {
326                Error::new(ErrorKind::Unexpected, "parse last modified time").set_source(e)
327            })?,
328        );
329        if let Some(v) = file.size {
330            meta = meta.with_content_length(v);
331        }
332        if let Some(v) = file.content_type {
333            meta = meta.with_content_type(v);
334        }
335
336        Ok(RpStat::new(meta))
337    }
338
339    async fn read(&self, path: &str, args: OpRead) -> Result<(RpRead, Self::Reader)> {
340        let res = self.core.get_by_path(path).await?;
341        let file: AliyunDriveFile =
342            serde_json::from_reader(res.reader()).map_err(new_json_serialize_error)?;
343        let resp = self.core.download(&file.file_id, args.range()).await?;
344
345        let status = resp.status();
346        match status {
347            StatusCode::OK | StatusCode::PARTIAL_CONTENT => {
348                Ok((RpRead::default(), resp.into_body()))
349            }
350            _ => {
351                let (part, mut body) = resp.into_parts();
352                let buf = body.to_buffer().await?;
353                Err(parse_error(Response::from_parts(part, buf)))
354            }
355        }
356    }
357
358    async fn delete(&self) -> Result<(RpDelete, Self::Deleter)> {
359        Ok((
360            RpDelete::default(),
361            oio::OneShotDeleter::new(AliyunDriveDeleter::new(self.core.clone())),
362        ))
363    }
364
365    async fn list(&self, path: &str, args: OpList) -> Result<(RpList, Self::Lister)> {
366        let parent = match self.core.get_by_path(path).await {
367            Err(err) if err.kind() == ErrorKind::NotFound => None,
368            Err(err) => return Err(err),
369            Ok(res) => {
370                let file: AliyunDriveFile =
371                    serde_json::from_reader(res.reader()).map_err(new_json_serialize_error)?;
372                Some(AliyunDriveParent {
373                    file_id: file.file_id,
374                    path: path.to_string(),
375                    updated_at: file.updated_at,
376                })
377            }
378        };
379
380        let l = AliyunDriveLister::new(self.core.clone(), parent, args.limit());
381
382        Ok((RpList::default(), oio::PageLister::new(l)))
383    }
384
385    async fn write(&self, path: &str, args: OpWrite) -> Result<(RpWrite, Self::Writer)> {
386        let parent_path = get_parent(path);
387        let parent_file_id = self.core.ensure_dir_exists(parent_path).await?;
388
389        // write can overwrite
390        match self.core.get_by_path(path).await {
391            Err(err) if err.kind() == ErrorKind::NotFound => {}
392            Err(err) => return Err(err),
393            Ok(res) => {
394                let file: AliyunDriveFile =
395                    serde_json::from_reader(res.reader()).map_err(new_json_serialize_error)?;
396                self.core.delete_path(&file.file_id).await?;
397            }
398        };
399
400        let writer =
401            AliyunDriveWriter::new(self.core.clone(), &parent_file_id, get_basename(path), args);
402
403        Ok((RpWrite::default(), writer))
404    }
405}
```
