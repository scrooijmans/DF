# 

opendal/services/compfs/

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
18use std::io::Cursor;
19use std::sync::Arc;
20
21use compio::dispatcher::Dispatcher;
22use compio::fs::OpenOptions;
23
24use super::COMPFS_SCHEME;
25use super::core::CompfsCore;
26use super::delete::CompfsDeleter;
27use super::lister::CompfsLister;
28use super::reader::CompfsReader;
29use super::writer::CompfsWriter;
30use crate::raw::oio::OneShotDeleter;
31use crate::raw::*;
32use crate::services::CompfsConfig;
33use crate::*;
34
35/// [`compio`]-based file system support.
36#[derive(Debug, Clone, Default)]
37pub struct CompfsBuilder {
38    pub(super) config: CompfsConfig,
39}
40
41impl CompfsBuilder {
42    /// Set root for Compfs
43    pub fn root(mut self, root: &str) -> Self {
44        self.config.root = if root.is_empty() {
45            None
46        } else {
47            Some(root.to_string())
48        };
49
50        self
51    }
52}
53
54impl Builder for CompfsBuilder {
55    type Config = CompfsConfig;
56
57    fn build(self) -> Result<impl Access> {
58        let root = match self.config.root {
59            Some(root) => Ok(root),
60            None => Err(Error::new(
61                ErrorKind::ConfigInvalid,
62                "root is not specified",
63            )),
64        }?;
65
66        // If root dir does not exist, we must create it.
67        if let Err(e) = std::fs::metadata(&root) {
68            if e.kind() == std::io::ErrorKind::NotFound {
69                std::fs::create_dir_all(&root).map_err(|e| {
70                    Error::new(ErrorKind::Unexpected, "create root dir failed")
71                        .with_operation("Builder::build")
72                        .with_context("root", root.as_str())
73                        .set_source(e)
74                })?;
75            }
76        }
77
78        let dispatcher = Dispatcher::new().map_err(|_| {
79            Error::new(
80                ErrorKind::Unexpected,
81                "failed to initiate compio dispatcher",
82            )
83        })?;
84        let core = CompfsCore {
85            info: {
86                let am = AccessorInfo::default();
87                am.set_scheme(COMPFS_SCHEME)
88                    .set_root(&root)
89                    .set_native_capability(Capability {
90                        stat: true,
91
92                        read: true,
93
94                        write: true,
95                        write_can_empty: true,
96                        write_can_multi: true,
97                        create_dir: true,
98                        delete: true,
99
100                        list: true,
101
102                        copy: true,
103                        rename: true,
104
105                        shared: true,
106
107                        ..Default::default()
108                    });
109
110                am.into()
111            },
112            root: root.into(),
113            dispatcher,
114            buf_pool: oio::PooledBuf::new(16),
115        };
116        Ok(CompfsBackend {
117            core: Arc::new(core),
118        })
119    }
120}
121
122#[derive(Debug)]
123pub struct CompfsBackend {
124    core: Arc<CompfsCore>,
125}
126
127impl Access for CompfsBackend {
128    type Reader = CompfsReader;
129    type Writer = CompfsWriter;
130    type Lister = Option<CompfsLister>;
131    type Deleter = OneShotDeleter<CompfsDeleter>;
132
133    fn info(&self) -> Arc<AccessorInfo> {
134        self.core.info.clone()
135    }
136
137    async fn create_dir(&self, path: &str, _: OpCreateDir) -> Result<RpCreateDir> {
138        let path = self.core.prepare_path(path);
139
140        self.core
141            .exec(move || async move { compio::fs::create_dir_all(path).await })
142            .await?;
143
144        Ok(RpCreateDir::default())
145    }
146
147    async fn stat(&self, path: &str, _: OpStat) -> Result<RpStat> {
148        let path = self.core.prepare_path(path);
149        let meta = self
150            .core
151            .exec(move || async move { compio::fs::metadata(path).await })
152            .await?;
153        let ty = meta.file_type();
154        let mode = if ty.is_dir() {
155            EntryMode::DIR
156        } else if ty.is_file() {
157            EntryMode::FILE
158        } else {
159            EntryMode::Unknown
160        };
161        let last_mod = Timestamp::try_from(meta.modified().map_err(new_std_io_error)?)?;
162        let ret = Metadata::new(mode)
163            .with_last_modified(last_mod)
164            .with_content_length(meta.len());
165        Ok(RpStat::new(ret))
166    }
167
168    async fn delete(&self) -> Result<(RpDelete, Self::Deleter)> {
169        Ok((
170            RpDelete::default(),
171            OneShotDeleter::new(CompfsDeleter::new(self.core.clone())),
172        ))
173    }
174
175    async fn copy(&self, from: &str, to: &str, _: OpCopy) -> Result<RpCopy> {
176        let from = self.core.prepare_path(from);
177        let to = self.core.prepare_path(to);
178
179        self.core
180            .exec(move || async move {
181                let from = OpenOptions::new().read(true).open(from).await?;
182                if let Some(parent) = to.parent() {
183                    compio::fs::create_dir_all(parent).await?;
184                }
185                let to = OpenOptions::new()
186                    .write(true)
187                    .create(true)
188                    .truncate(true)
189                    .open(to)
190                    .await?;
191
192                let (mut from, mut to) = (Cursor::new(from), Cursor::new(to));
193                compio::io::copy(&mut from, &mut to).await?;
194
195                Ok(())
196            })
197            .await?;
198
199        Ok(RpCopy::default())
200    }
201
202    async fn rename(&self, from: &str, to: &str, _: OpRename) -> Result<RpRename> {
203        let from = self.core.prepare_path(from);
204        let to = self.core.prepare_path(to);
205
206        self.core
207            .exec(move || async move {
208                if let Some(parent) = to.parent() {
209                    compio::fs::create_dir_all(parent).await?;
210                }
211                compio::fs::rename(from, to).await
212            })
213            .await?;
214
215        Ok(RpRename::default())
216    }
217
218    async fn read(&self, path: &str, op: OpRead) -> Result<(RpRead, Self::Reader)> {
219        let path = self.core.prepare_path(path);
220
221        let file = self
222            .core
223            .exec(|| async move { compio::fs::OpenOptions::new().read(true).open(&path).await })
224            .await?;
225
226        let r = CompfsReader::new(self.core.clone(), file, op.range());
227        Ok((RpRead::new(), r))
228    }
229
230    async fn write(&self, path: &str, args: OpWrite) -> Result<(RpWrite, Self::Writer)> {
231        let path = self.core.prepare_path(path);
232        let append = args.append();
233        let file = self
234            .core
235            .exec(move || async move {
236                if let Some(parent) = path.parent() {
237                    compio::fs::create_dir_all(parent).await?;
238                }
239                let file = compio::fs::OpenOptions::new()
240                    .create(true)
241                    .write(true)
242                    .truncate(!append)
243                    .open(path)
244                    .await?;
245                let mut file = Cursor::new(file);
246                if append {
247                    let len = file.get_ref().metadata().await?.len();
248                    file.set_position(len);
249                }
250                Ok(file)
251            })
252            .await?;
253
254        let w = CompfsWriter::new(self.core.clone(), file);
255        Ok((RpWrite::new(), w))
256    }
257
258    async fn list(&self, path: &str, _: OpList) -> Result<(RpList, Self::Lister)> {
259        let path = self.core.prepare_path(path);
260
261        let read_dir = match self
262            .core
263            .exec_blocking({
264                let path = path.clone();
265                move || std::fs::read_dir(path)
266            })
267            .await?
268        {
269            Ok(rd) => rd,
270            Err(e) => {
271                return if e.kind() == std::io::ErrorKind::NotFound {
272                    Ok((RpList::default(), None))
273                } else {
274                    Err(new_std_io_error(e))
275                };
276            }
277        };
278
279        let lister = CompfsLister::new(self.core.clone(), &path, read_dir);
280        Ok((RpList::default(), Some(lister)))
281    }
282}
```
