# 

opendal/services/monoiofs/

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
18use monoio::fs::OpenOptions;
19use std::fmt::Debug;
20use std::io;
21use std::path::PathBuf;
22use std::sync::Arc;
23
24use super::core::BUFFER_SIZE;
25use super::core::MonoiofsCore;
26use super::delete::MonoiofsDeleter;
27use super::reader::MonoiofsReader;
28use super::writer::MonoiofsWriter;
29use crate::raw::*;
30use crate::services::MonoiofsConfig;
31use crate::*;
32
33/// File system support via [`monoio`].
34#[doc = include_str!("docs.md")]
35#[derive(Default, Debug)]
36pub struct MonoiofsBuilder {
37    pub(super) config: MonoiofsConfig,
38}
39
40impl MonoiofsBuilder {
41    /// Set root of this backend.
42    ///
43    /// All operations will happen under this root.
44    pub fn root(mut self, root: &str) -> Self {
45        self.config.root = if root.is_empty() {
46            None
47        } else {
48            Some(root.to_string())
49        };
50        self
51    }
52}
53
54impl Builder for MonoiofsBuilder {
55    type Config = MonoiofsConfig;
56
57    fn build(self) -> Result<impl Access> {
58        let root = self.config.root.map(PathBuf::from).ok_or(
59            Error::new(ErrorKind::ConfigInvalid, "root is not specified")
60                .with_operation("Builder::build"),
61        )?;
62        if let Err(e) = std::fs::metadata(&root) {
63            if e.kind() == io::ErrorKind::NotFound {
64                std::fs::create_dir_all(&root).map_err(|e| {
65                    Error::new(ErrorKind::Unexpected, "create root dir failed")
66                        .with_operation("Builder::build")
67                        .with_context("root", root.to_string_lossy())
68                        .set_source(e)
69                })?;
70            }
71        }
72        let root = root.canonicalize().map_err(|e| {
73            Error::new(
74                ErrorKind::Unexpected,
75                "canonicalize of root directory failed",
76            )
77            .with_operation("Builder::build")
78            .with_context("root", root.to_string_lossy())
79            .set_source(e)
80        })?;
81        let worker_threads = 1; // TODO: test concurrency and default to available_parallelism and bind cpu
82        let io_uring_entries = 1024;
83        Ok(MonoiofsBackend {
84            core: Arc::new(MonoiofsCore::new(root, worker_threads, io_uring_entries)),
85        })
86    }
87}
88
89#[derive(Debug, Clone)]
90pub struct MonoiofsBackend {
91    core: Arc<MonoiofsCore>,
92}
93
94impl Access for MonoiofsBackend {
95    type Reader = MonoiofsReader;
96    type Writer = MonoiofsWriter;
97    type Lister = ();
98    type Deleter = oio::OneShotDeleter<MonoiofsDeleter>;
99
100    fn info(&self) -> Arc<AccessorInfo> {
101        self.core.info.clone()
102    }
103
104    async fn stat(&self, path: &str, _args: OpStat) -> Result<RpStat> {
105        let path = self.core.prepare_path(path);
106        let meta = self
107            .core
108            .dispatch(move || monoio::fs::metadata(path))
109            .await
110            .map_err(new_std_io_error)?;
111        let mode = if meta.is_dir() {
112            EntryMode::DIR
113        } else if meta.is_file() {
114            EntryMode::FILE
115        } else {
116            EntryMode::Unknown
117        };
118        let m = Metadata::new(mode)
119            .with_content_length(meta.len())
120            .with_last_modified(Timestamp::try_from(
121                meta.modified().map_err(new_std_io_error)?,
122            )?);
123        Ok(RpStat::new(m))
124    }
125
126    async fn read(&self, path: &str, args: OpRead) -> Result<(RpRead, Self::Reader)> {
127        let path = self.core.prepare_path(path);
128        let reader = MonoiofsReader::new(self.core.clone(), path, args.range()).await?;
129        Ok((RpRead::default(), reader))
130    }
131
132    async fn write(&self, path: &str, args: OpWrite) -> Result<(RpWrite, Self::Writer)> {
133        let path = self.core.prepare_write_path(path).await?;
134        let writer = MonoiofsWriter::new(self.core.clone(), path, args.append()).await?;
135        Ok((RpWrite::default(), writer))
136    }
137
138    async fn delete(&self) -> Result<(RpDelete, Self::Deleter)> {
139        Ok((
140            RpDelete::default(),
141            oio::OneShotDeleter::new(MonoiofsDeleter::new(self.core.clone())),
142        ))
143    }
144
145    async fn rename(&self, from: &str, to: &str, _args: OpRename) -> Result<RpRename> {
146        let from = self.core.prepare_path(from);
147        // ensure file exists
148        self.core
149            .dispatch({
150                let from = from.clone();
151                move || monoio::fs::metadata(from)
152            })
153            .await
154            .map_err(new_std_io_error)?;
155        let to = self.core.prepare_write_path(to).await?;
156        self.core
157            .dispatch(move || monoio::fs::rename(from, to))
158            .await
159            .map_err(new_std_io_error)?;
160        Ok(RpRename::default())
161    }
162
163    async fn create_dir(&self, path: &str, _args: OpCreateDir) -> Result<RpCreateDir> {
164        let path = self.core.prepare_path(path);
165        self.core
166            .dispatch(move || monoio::fs::create_dir_all(path))
167            .await
168            .map_err(new_std_io_error)?;
169        Ok(RpCreateDir::default())
170    }
171
172    async fn copy(&self, from: &str, to: &str, _args: OpCopy) -> Result<RpCopy> {
173        let from = self.core.prepare_path(from);
174        // ensure file exists
175        self.core
176            .dispatch({
177                let from = from.clone();
178                move || monoio::fs::metadata(from)
179            })
180            .await
181            .map_err(new_std_io_error)?;
182        let to = self.core.prepare_write_path(to).await?;
183        self.core
184            .dispatch({
185                let core = self.core.clone();
186                move || async move {
187                    let from = OpenOptions::new().read(true).open(from).await?;
188                    let to = OpenOptions::new()
189                        .write(true)
190                        .create(true)
191                        .truncate(true)
192                        .open(to)
193                        .await?;
194
195                    // AsyncReadRent and AsyncWriteRent is not implemented
196                    // for File, so we can't write this:
197                    // monoio::io::copy(&mut from, &mut to).await?;
198
199                    let mut pos = 0;
200                    // allocate and resize buffer
201                    let mut buf = core.buf_pool.get();
202                    // set capacity of buf to exact size to avoid excessive read
203                    buf.reserve(BUFFER_SIZE);
204                    let _ = buf.split_off(BUFFER_SIZE);
205
206                    loop {
207                        let result;
208                        (result, buf) = from.read_at(buf, pos).await;
209                        if result? == 0 {
210                            // EOF
211                            break;
212                        }
213                        let result;
214                        (result, buf) = to.write_all_at(buf, pos).await;
215                        result?;
216                        pos += buf.len() as u64;
217                        buf.clear();
218                    }
219                    core.buf_pool.put(buf);
220                    Ok(())
221                }
222            })
223            .await
224            .map_err(new_std_io_error)?;
225        Ok(RpCopy::default())
226    }
227}
```
