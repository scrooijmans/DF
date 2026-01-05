# 

opendal/services/fs/

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
18use std::path::PathBuf;
19use std::sync::Arc;
20
21use log::debug;
22
23use super::FS_SCHEME;
24use super::core::*;
25use super::delete::FsDeleter;
26use super::lister::FsLister;
27use super::reader::FsReader;
28use super::writer::FsWriter;
29use super::writer::FsWriters;
30use crate::raw::*;
31use crate::services::FsConfig;
32use crate::*;
33
34/// POSIX file system support.
35#[doc = include_str!("docs.md")]
36#[derive(Default, Debug)]
37pub struct FsBuilder {
38    pub(super) config: FsConfig,
39}
40
41impl FsBuilder {
42    /// Set root for backend.
43    pub fn root(mut self, root: &str) -> Self {
44        self.config.root = if root.is_empty() {
45            None
46        } else {
47            Some(root.to_string())
48        };
49
50        self
51    }
52
53    /// Set temp dir for atomic write.
54    ///
55    /// # Notes
56    ///
57    /// - When append is enabled, we will not use atomic write
58    ///   to avoid data loss and performance issue.
59    pub fn atomic_write_dir(mut self, dir: &str) -> Self {
60        if !dir.is_empty() {
61            self.config.atomic_write_dir = Some(dir.to_string());
62        }
63
64        self
65    }
66}
67
68impl Builder for FsBuilder {
69    type Config = FsConfig;
70
71    fn build(self) -> Result<impl Access> {
72        debug!("backend build started: {:?}", &self);
73
74        let root = match self.config.root.map(PathBuf::from) {
75            Some(root) => Ok(root),
76            None => Err(Error::new(
77                ErrorKind::ConfigInvalid,
78                "root is not specified",
79            )),
80        }?;
81        debug!("backend use root {}", root.to_string_lossy());
82
83        // If root dir is not exist, we must create it.
84        if let Err(e) = std::fs::metadata(&root) {
85            if e.kind() == std::io::ErrorKind::NotFound {
86                std::fs::create_dir_all(&root).map_err(|e| {
87                    Error::new(ErrorKind::Unexpected, "create root dir failed")
88                        .with_operation("Builder::build")
89                        .with_context("root", root.to_string_lossy())
90                        .set_source(e)
91                })?;
92            }
93        }
94
95        let atomic_write_dir = self.config.atomic_write_dir.map(PathBuf::from);
96
97        // If atomic write dir is not exist, we must create it.
98        if let Some(d) = &atomic_write_dir {
99            if let Err(e) = std::fs::metadata(d) {
100                if e.kind() == std::io::ErrorKind::NotFound {
101                    std::fs::create_dir_all(d).map_err(|e| {
102                        Error::new(ErrorKind::Unexpected, "create atomic write dir failed")
103                            .with_operation("Builder::build")
104                            .with_context("atomic_write_dir", d.to_string_lossy())
105                            .set_source(e)
106                    })?;
107                }
108            }
109        }
110
111        // Canonicalize the root directory. This should work since we already know that we can
112        // get the metadata of the path.
113        let root = root.canonicalize().map_err(|e| {
114            Error::new(
115                ErrorKind::Unexpected,
116                "canonicalize of root directory failed",
117            )
118            .set_source(e)
119        })?;
120
121        // Canonicalize the atomic_write_dir directory. This should work since we already know that
122        // we can get the metadata of the path.
123        let atomic_write_dir = atomic_write_dir
124            .map(|p| {
125                p.canonicalize().map(Some).map_err(|e| {
126                    Error::new(
127                        ErrorKind::Unexpected,
128                        "canonicalize of atomic_write_dir directory failed",
129                    )
130                    .with_operation("Builder::build")
131                    .with_context("root", root.to_string_lossy())
132                    .set_source(e)
133                })
134            })
135            .unwrap_or(Ok(None))?;
136
137        Ok(FsBackend {
138            core: Arc::new(FsCore {
139                info: {
140                    let am = AccessorInfo::default();
141                    am.set_scheme(FS_SCHEME)
142                        .set_root(&root.to_string_lossy())
143                        .set_native_capability(Capability {
144                            stat: true,
145
146                            read: true,
147
148                            write: true,
149                            write_can_empty: true,
150                            write_can_append: true,
151                            write_can_multi: true,
152                            write_with_if_not_exists: true,
153
154                            create_dir: true,
155                            delete: true,
156
157                            list: true,
158
159                            copy: true,
160                            rename: true,
161
162                            shared: true,
163
164                            ..Default::default()
165                        });
166
167                    am.into()
168                },
169                root,
170                atomic_write_dir,
171                buf_pool: oio::PooledBuf::new(16).with_initial_capacity(256 * 1024),
172            }),
173        })
174    }
175}
176
177/// Backend is used to serve `Accessor` support for posix-like fs.
178#[derive(Debug, Clone)]
179pub struct FsBackend {
180    core: Arc<FsCore>,
181}
182
183impl Access for FsBackend {
184    type Reader = FsReader<tokio::fs::File>;
185    type Writer = FsWriters;
186    type Lister = Option<FsLister<tokio::fs::ReadDir>>;
187    type Deleter = oio::OneShotDeleter<FsDeleter>;
188
189    fn info(&self) -> Arc<AccessorInfo> {
190        self.core.info.clone()
191    }
192
193    async fn create_dir(&self, path: &str, _: OpCreateDir) -> Result<RpCreateDir> {
194        self.core.fs_create_dir(path).await?;
195        Ok(RpCreateDir::default())
196    }
197
198    async fn stat(&self, path: &str, _: OpStat) -> Result<RpStat> {
199        let m = self.core.fs_stat(path).await?;
200        Ok(RpStat::new(m))
201    }
202
203    /// # Notes
204    ///
205    /// There are three ways to get the total file length:
206    ///
207    /// - call std::fs::metadata directly and then open. (400ns)
208    /// - open file first, and then use `f.metadata()` (300ns)
209    /// - open file first, and then use `seek`. (100ns)
210    ///
211    /// Benchmark could be found [here](https://gist.github.com/Xuanwo/48f9cfbc3022ea5f865388bb62e1a70f)
212    async fn read(&self, path: &str, args: OpRead) -> Result<(RpRead, Self::Reader)> {
213        let f = self.core.fs_read(path, &args).await?;
214        let r = FsReader::new(
215            self.core.clone(),
216            f,
217            args.range().size().unwrap_or(u64::MAX) as _,
218        );
219        Ok((RpRead::new(), r))
220    }
221
222    async fn write(&self, path: &str, op: OpWrite) -> Result<(RpWrite, Self::Writer)> {
223        let is_append = op.append();
224        let concurrent = op.concurrent();
225
226        let writer = FsWriter::create(self.core.clone(), path, op).await?;
227
228        let writer = if is_append {
229            FsWriters::One(writer)
230        } else {
231            FsWriters::Two(oio::PositionWriter::new(
232                self.info().clone(),
233                writer,
234                concurrent,
235            ))
236        };
237
238        Ok((RpWrite::default(), writer))
239    }
240
241    async fn delete(&self) -> Result<(RpDelete, Self::Deleter)> {
242        Ok((
243            RpDelete::default(),
244            oio::OneShotDeleter::new(FsDeleter::new(self.core.clone())),
245        ))
246    }
247
248    async fn list(&self, path: &str, _: OpList) -> Result<(RpList, Self::Lister)> {
249        match self.core.fs_list(path).await? {
250            Some(f) => {
251                let rd = FsLister::new(&self.core.root, path, f);
252                Ok((RpList::default(), Some(rd)))
253            }
254            None => Ok((RpList::default(), None)),
255        }
256    }
257
258    async fn copy(&self, from: &str, to: &str, _args: OpCopy) -> Result<RpCopy> {
259        self.core.fs_copy(from, to).await?;
260        Ok(RpCopy::default())
261    }
262
263    async fn rename(&self, from: &str, to: &str, _args: OpRename) -> Result<RpRename> {
264        self.core.fs_rename(from, to).await?;
265        Ok(RpRename::default())
266    }
267}
```
