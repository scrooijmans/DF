# 

opendal/services/hdfs/

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
20use std::io;
21use std::io::SeekFrom;
22use std::path::PathBuf;
23use std::sync::Arc;
24
25use log::debug;
26
27use super::HDFS_SCHEME;
28use super::delete::HdfsDeleter;
29use super::lister::HdfsLister;
30use super::reader::HdfsReader;
31use super::writer::HdfsWriter;
32use crate::raw::*;
33use crate::services::HdfsConfig;
34use crate::*;
35
36#[doc = include_str!("docs.md")]
37#[derive(Default)]
38pub struct HdfsBuilder {
39    pub(super) config: HdfsConfig,
40}
41
42impl Debug for HdfsBuilder {
43    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
44        f.debug_struct("HdfsBuilder")
45            .field("config", &self.config)
46            .finish()
47    }
48}
49
50impl HdfsBuilder {
51    /// Set root of this backend.
52    ///
53    /// All operations will happen under this root.
54    pub fn root(mut self, root: &str) -> Self {
55        self.config.root = if root.is_empty() {
56            None
57        } else {
58            Some(root.to_string())
59        };
60
61        self
62    }
63
64    /// Set name_node of this backend.
65    ///
66    /// Valid format including:
67    ///
68    /// - `default`: using the default setting based on hadoop config.
69    /// - `hdfs://127.0.0.1:9000`: connect to hdfs cluster.
70    pub fn name_node(mut self, name_node: &str) -> Self {
71        if !name_node.is_empty() {
72            self.config.name_node = Some(name_node.to_string())
73        }
74
75        self
76    }
77
78    /// Set kerberos_ticket_cache_path of this backend
79    ///
80    /// This should be configured when kerberos is enabled.
81    pub fn kerberos_ticket_cache_path(mut self, kerberos_ticket_cache_path: &str) -> Self {
82        if !kerberos_ticket_cache_path.is_empty() {
83            self.config.kerberos_ticket_cache_path = Some(kerberos_ticket_cache_path.to_string())
84        }
85        self
86    }
87
88    /// Set user of this backend
89    pub fn user(mut self, user: &str) -> Self {
90        if !user.is_empty() {
91            self.config.user = Some(user.to_string())
92        }
93        self
94    }
95
96    /// Enable append capacity of this backend.
97    ///
98    /// This should be disabled when HDFS runs in non-distributed mode.
99    pub fn enable_append(mut self, enable_append: bool) -> Self {
100        self.config.enable_append = enable_append;
101        self
102    }
103
104    /// Set temp dir for atomic write.
105    ///
106    /// # Notes
107    ///
108    /// - When append is enabled, we will not use atomic write
109    ///   to avoid data loss and performance issue.
110    pub fn atomic_write_dir(mut self, dir: &str) -> Self {
111        self.config.atomic_write_dir = if dir.is_empty() {
112            None
113        } else {
114            Some(String::from(dir))
115        };
116        self
117    }
118}
119
120impl Builder for HdfsBuilder {
121    type Config = HdfsConfig;
122
123    fn build(self) -> Result<impl Access> {
124        debug!("backend build started: {:?}", &self);
125
126        let name_node = match &self.config.name_node {
127            Some(v) => v,
128            None => {
129                return Err(Error::new(ErrorKind::ConfigInvalid, "name node is empty")
130                    .with_context("service", Scheme::Hdfs));
131            }
132        };
133
134        let root = normalize_root(&self.config.root.unwrap_or_default());
135        debug!("backend use root {root}");
136
137        let mut builder = hdrs::ClientBuilder::new(name_node);
138        if let Some(ticket_cache_path) = &self.config.kerberos_ticket_cache_path {
139            builder = builder.with_kerberos_ticket_cache_path(ticket_cache_path.as_str());
140        }
141        if let Some(user) = &self.config.user {
142            builder = builder.with_user(user.as_str());
143        }
144
145        let client = builder.connect().map_err(new_std_io_error)?;
146
147        // Create root dir if not exist.
148        if let Err(e) = client.metadata(&root) {
149            if e.kind() == io::ErrorKind::NotFound {
150                debug!("root {root} is not exist, creating now");
151
152                client.create_dir(&root).map_err(new_std_io_error)?
153            }
154        }
155
156        let atomic_write_dir = self.config.atomic_write_dir;
157
158        // If atomic write dir is not exist, we must create it.
159        if let Some(d) = &atomic_write_dir {
160            if let Err(e) = client.metadata(d) {
161                if e.kind() == io::ErrorKind::NotFound {
162                    client.create_dir(d).map_err(new_std_io_error)?
163                }
164            }
165        }
166
167        Ok(HdfsBackend {
168            info: {
169                let am = AccessorInfo::default();
170                am.set_scheme(HDFS_SCHEME)
171                    .set_root(&root)
172                    .set_native_capability(Capability {
173                        stat: true,
174
175                        read: true,
176
177                        write: true,
178                        write_can_append: self.config.enable_append,
179
180                        create_dir: true,
181                        delete: true,
182
183                        list: true,
184
185                        rename: true,
186
187                        shared: true,
188
189                        ..Default::default()
190                    });
191
192                am.into()
193            },
194            root,
195            atomic_write_dir,
196            client: Arc::new(client),
197        })
198    }
199}
200
201/// Backend for hdfs services.
202#[derive(Debug, Clone)]
203pub struct HdfsBackend {
204    pub info: Arc<AccessorInfo>,
205    pub root: String,
206    atomic_write_dir: Option<String>,
207    pub client: Arc<hdrs::Client>,
208}
209
210/// hdrs::Client is thread-safe.
211unsafe impl Send for HdfsBackend {}
212unsafe impl Sync for HdfsBackend {}
213
214impl Access for HdfsBackend {
215    type Reader = HdfsReader<hdrs::AsyncFile>;
216    type Writer = HdfsWriter<hdrs::AsyncFile>;
217    type Lister = Option<HdfsLister>;
218    type Deleter = oio::OneShotDeleter<HdfsDeleter>;
219
220    fn info(&self) -> Arc<AccessorInfo> {
221        self.info.clone()
222    }
223
224    async fn create_dir(&self, path: &str, _: OpCreateDir) -> Result<RpCreateDir> {
225        let p = build_rooted_abs_path(&self.root, path);
226
227        self.client.create_dir(&p).map_err(new_std_io_error)?;
228
229        Ok(RpCreateDir::default())
230    }
231
232    async fn stat(&self, path: &str, _: OpStat) -> Result<RpStat> {
233        let p = build_rooted_abs_path(&self.root, path);
234
235        let meta = self.client.metadata(&p).map_err(new_std_io_error)?;
236
237        let mode = if meta.is_dir() {
238            EntryMode::DIR
239        } else if meta.is_file() {
240            EntryMode::FILE
241        } else {
242            EntryMode::Unknown
243        };
244        let mut m = Metadata::new(mode);
245        m.set_content_length(meta.len());
246        m.set_last_modified(Timestamp::try_from(meta.modified())?);
247
248        Ok(RpStat::new(m))
249    }
250
251    async fn read(&self, path: &str, args: OpRead) -> Result<(RpRead, Self::Reader)> {
252        let p = build_rooted_abs_path(&self.root, path);
253
254        let client = self.client.clone();
255        let mut f = client
256            .open_file()
257            .read(true)
258            .async_open(&p)
259            .await
260            .map_err(new_std_io_error)?;
261
262        if args.range().offset() != 0 {
263            use futures::AsyncSeekExt;
264
265            f.seek(SeekFrom::Start(args.range().offset()))
266                .await
267                .map_err(new_std_io_error)?;
268        }
269
270        Ok((
271            RpRead::new(),
272            HdfsReader::new(f, args.range().size().unwrap_or(u64::MAX) as _),
273        ))
274    }
275
276    async fn write(&self, path: &str, op: OpWrite) -> Result<(RpWrite, Self::Writer)> {
277        let target_path = build_rooted_abs_path(&self.root, path);
278        let mut initial_size = 0;
279        let target_exists = match self.client.metadata(&target_path) {
280            Ok(meta) => {
281                initial_size = meta.len();
282                true
283            }
284            Err(err) => {
285                if err.kind() != io::ErrorKind::NotFound {
286                    return Err(new_std_io_error(err));
287                }
288                false
289            }
290        };
291
292        let should_append = op.append() && target_exists;
293        let tmp_path = self.atomic_write_dir.as_ref().and_then(|atomic_write_dir| {
294            // If the target file exists, we should append to the end of it directly.
295            (!should_append).then_some(build_rooted_abs_path(
296                atomic_write_dir,
297                &build_tmp_path_of(path),
298            ))
299        });
300
301        if !target_exists {
302            let parent = get_parent(&target_path);
303            self.client.create_dir(parent).map_err(new_std_io_error)?;
304        }
305        if !should_append {
306            initial_size = 0;
307        }
308
309        let mut open_options = self.client.open_file();
310        open_options.create(true);
311        if should_append {
312            open_options.append(true);
313        } else {
314            open_options.write(true);
315        }
316
317        let f = open_options
318            .async_open(tmp_path.as_ref().unwrap_or(&target_path))
319            .await
320            .map_err(new_std_io_error)?;
321
322        Ok((
323            RpWrite::new(),
324            HdfsWriter::new(
325                target_path,
326                tmp_path,
327                f,
328                Arc::clone(&self.client),
329                target_exists,
330                initial_size,
331            ),
332        ))
333    }
334
335    async fn delete(&self) -> Result<(RpDelete, Self::Deleter)> {
336        Ok((
337            RpDelete::default(),
338            oio::OneShotDeleter::new(HdfsDeleter::new(Arc::new(self.clone()))),
339        ))
340    }
341
342    async fn list(&self, path: &str, _: OpList) -> Result<(RpList, Self::Lister)> {
343        let p = build_rooted_abs_path(&self.root, path);
344
345        let f = match self.client.read_dir(&p) {
346            Ok(f) => f,
347            Err(e) => {
348                return if e.kind() == io::ErrorKind::NotFound {
349                    Ok((RpList::default(), None))
350                } else {
351                    Err(new_std_io_error(e))
352                };
353            }
354        };
355
356        let rd = HdfsLister::new(&self.root, f, path);
357
358        Ok((RpList::default(), Some(rd)))
359    }
360
361    async fn rename(&self, from: &str, to: &str, _args: OpRename) -> Result<RpRename> {
362        let from_path = build_rooted_abs_path(&self.root, from);
363        self.client.metadata(&from_path).map_err(new_std_io_error)?;
364
365        let to_path = build_rooted_abs_path(&self.root, to);
366        let result = self.client.metadata(&to_path);
367        match result {
368            Err(err) => {
369                // Early return if other error happened.
370                if err.kind() != io::ErrorKind::NotFound {
371                    return Err(new_std_io_error(err));
372                }
373
374                let parent = PathBuf::from(&to_path)
375                    .parent()
376                    .ok_or_else(|| {
377                        Error::new(
378                            ErrorKind::Unexpected,
379                            "path should have parent but not, it must be malformed",
380                        )
381                        .with_context("input", &to_path)
382                    })?
383                    .to_path_buf();
384
385                self.client
386                    .create_dir(&parent.to_string_lossy())
387                    .map_err(new_std_io_error)?;
388            }
389            Ok(metadata) => {
390                if metadata.is_file() {
391                    self.client
392                        .remove_file(&to_path)
393                        .map_err(new_std_io_error)?;
394                } else {
395                    return Err(Error::new(ErrorKind::IsADirectory, "path should be a file")
396                        .with_context("input", &to_path));
397                }
398            }
399        }
400
401        self.client
402            .rename_file(&from_path, &to_path)
403            .map_err(new_std_io_error)?;
404
405        Ok(RpRename::new())
406    }
407}
```
