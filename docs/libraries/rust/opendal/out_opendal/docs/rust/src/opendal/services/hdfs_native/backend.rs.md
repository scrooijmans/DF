# 

opendal/services/hdfs_native/

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
22use hdfs_native::HdfsError;
23use hdfs_native::WriteOptions;
24use log::debug;
25
26use super::HDFS_NATIVE_SCHEME;
27use super::delete::HdfsNativeDeleter;
28use super::error::parse_hdfs_error;
29use super::lister::HdfsNativeLister;
30use super::reader::HdfsNativeReader;
31use super::writer::HdfsNativeWriter;
32use crate::raw::*;
33use crate::services::HdfsNativeConfig;
34use crate::*;
35/// [Hadoop Distributed File System (HDFSâ¢)](https://hadoop.apache.org/) support.
36/// Using [Native Rust HDFS client](https://github.com/Kimahriman/hdfs-native).
37#[doc = include_str!("docs.md")]
38#[derive(Default)]
39pub struct HdfsNativeBuilder {
40    pub(super) config: HdfsNativeConfig,
41}
42
43impl Debug for HdfsNativeBuilder {
44    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
45        f.debug_struct("HdfsNativeBuilder")
46            .field("config", &self.config)
47            .finish()
48    }
49}
50
51impl HdfsNativeBuilder {
52    /// Set root of this backend.
53    ///
54    /// All operations will happen under this root.
55    pub fn root(mut self, root: &str) -> Self {
56        self.config.root = if root.is_empty() {
57            None
58        } else {
59            Some(root.to_string())
60        };
61
62        self
63    }
64
65    /// Set name_node of this backend.
66    ///
67    /// Valid format including:
68    ///
69    /// - `default`: using the default setting based on hadoop config.
70    /// - `hdfs://127.0.0.1:9000`: connect to hdfs cluster.
71    pub fn name_node(mut self, name_node: &str) -> Self {
72        if !name_node.is_empty() {
73            // Trim trailing `/` so that we can accept `http://127.0.0.1:9000/`
74            self.config.name_node = Some(name_node.trim_end_matches('/').to_string())
75        }
76
77        self
78    }
79
80    /// Enable append capacity of this backend.
81    ///
82    /// This should be disabled when HDFS runs in non-distributed mode.
83    pub fn enable_append(mut self, enable_append: bool) -> Self {
84        self.config.enable_append = enable_append;
85        self
86    }
87}
88
89impl Builder for HdfsNativeBuilder {
90    type Config = HdfsNativeConfig;
91
92    fn build(self) -> Result<impl Access> {
93        debug!("backend build started: {:?}", &self);
94
95        let name_node = match &self.config.name_node {
96            Some(v) => v,
97            None => {
98                return Err(Error::new(ErrorKind::ConfigInvalid, "name_node is empty")
99                    .with_context("service", Scheme::HdfsNative));
100            }
101        };
102
103        let root = normalize_root(&self.config.root.unwrap_or_default());
104        debug!("backend use root {root}");
105
106        let client = hdfs_native::ClientBuilder::new()
107            .with_url(name_node)
108            .build()
109            .map_err(parse_hdfs_error)?;
110
111        // need to check if root dir exists, create if not
112        Ok(HdfsNativeBackend {
113            root,
114            client: Arc::new(client),
115            enable_append: self.config.enable_append,
116        })
117    }
118}
119
120// #[inline]
121// fn tmp_file_of(path: &str) -> String {
122//     let name = get_basename(path);
123//     let uuid = Uuid::new_v4().to_string();
124
125//     format!("{name}.{uuid}")
126// }
127
128/// Backend for hdfs-native services.
129#[derive(Debug, Clone)]
130pub struct HdfsNativeBackend {
131    pub root: String,
132    pub client: Arc<hdfs_native::Client>,
133    enable_append: bool,
134}
135
136/// hdfs_native::Client is thread-safe.
137unsafe impl Send for HdfsNativeBackend {}
138unsafe impl Sync for HdfsNativeBackend {}
139
140impl Access for HdfsNativeBackend {
141    type Reader = HdfsNativeReader;
142    type Writer = HdfsNativeWriter;
143    type Lister = Option<HdfsNativeLister>;
144    type Deleter = oio::OneShotDeleter<HdfsNativeDeleter>;
145
146    fn info(&self) -> Arc<AccessorInfo> {
147        let am = AccessorInfo::default();
148        am.set_scheme(HDFS_NATIVE_SCHEME)
149            .set_root(&self.root)
150            .set_native_capability(Capability {
151                stat: true,
152
153                read: true,
154
155                write: true,
156                write_can_append: self.enable_append,
157
158                create_dir: true,
159                delete: true,
160
161                list: true,
162
163                rename: true,
164
165                shared: true,
166
167                ..Default::default()
168            });
169
170        am.into()
171    }
172
173    async fn create_dir(&self, path: &str, _args: OpCreateDir) -> Result<RpCreateDir> {
174        let p = build_rooted_abs_path(&self.root, path);
175
176        self.client
177            .mkdirs(&p, 0o777, true)
178            .await
179            .map_err(parse_hdfs_error)?;
180
181        Ok(RpCreateDir::default())
182    }
183
184    async fn stat(&self, path: &str, _args: OpStat) -> Result<RpStat> {
185        let p = build_rooted_abs_path(&self.root, path);
186
187        let status: hdfs_native::client::FileStatus = self
188            .client
189            .get_file_info(&p)
190            .await
191            .map_err(parse_hdfs_error)?;
192
193        let mode = if status.isdir {
194            EntryMode::DIR
195        } else {
196            EntryMode::FILE
197        };
198
199        let mut metadata = Metadata::new(mode);
200        metadata
201            .set_last_modified(Timestamp::from_millisecond(
202                status.modification_time as i64,
203            )?)
204            .set_content_length(status.length as u64);
205
206        Ok(RpStat::new(metadata))
207    }
208
209    async fn read(&self, path: &str, args: OpRead) -> Result<(RpRead, Self::Reader)> {
210        let p = build_rooted_abs_path(&self.root, path);
211
212        let f = self.client.read(&p).await.map_err(parse_hdfs_error)?;
213
214        let r = HdfsNativeReader::new(
215            f,
216            args.range().offset() as _,
217            args.range().size().unwrap_or(u64::MAX) as _,
218        );
219
220        Ok((RpRead::new(), r))
221    }
222
223    async fn write(&self, path: &str, args: OpWrite) -> Result<(RpWrite, Self::Writer)> {
224        let target_path = build_rooted_abs_path(&self.root, path);
225        let mut initial_size = 0;
226
227        let target_exists = match self.client.get_file_info(&target_path).await {
228            Ok(status) => {
229                initial_size = status.length as u64;
230                true
231            }
232            Err(err) => match &err {
233                HdfsError::FileNotFound(_) => false,
234                _ => return Err(parse_hdfs_error(err)),
235            },
236        };
237
238        let f = if target_exists {
239            if args.append() {
240                assert!(self.enable_append, "append is not enabled");
241                self.client
242                    .append(&target_path)
243                    .await
244                    .map_err(parse_hdfs_error)?
245            } else {
246                initial_size = 0;
247                self.client
248                    .create(&target_path, WriteOptions::default().overwrite(true))
249                    .await
250                    .map_err(parse_hdfs_error)?
251            }
252        } else {
253            initial_size = 0;
254            self.client
255                .create(&target_path, WriteOptions::default())
256                .await
257                .map_err(parse_hdfs_error)?
258        };
259
260        Ok((RpWrite::new(), HdfsNativeWriter::new(f, initial_size)))
261    }
262
263    async fn delete(&self) -> Result<(RpDelete, Self::Deleter)> {
264        Ok((
265            RpDelete::default(),
266            oio::OneShotDeleter::new(HdfsNativeDeleter::new(Arc::new(self.clone()))),
267        ))
268    }
269
270    async fn list(&self, path: &str, _args: OpList) -> Result<(RpList, Self::Lister)> {
271        let p: String = build_rooted_abs_path(&self.root, path);
272
273        let isdir = match self.client.get_file_info(&p).await {
274            Ok(status) => status.isdir,
275            Err(err) => {
276                return match &err {
277                    HdfsError::FileNotFound(_) => Ok((RpList::default(), None)),
278                    _ => Err(parse_hdfs_error(err)),
279                };
280            }
281        };
282        let current_path = if isdir {
283            if !path.ends_with("/") {
284                Some(path.to_string() + "/")
285            } else {
286                Some(path.to_string())
287            }
288        } else {
289            None
290        };
291
292        Ok((
293            RpList::default(),
294            Some(HdfsNativeLister::new(
295                &self.root,
296                &self.client,
297                &p,
298                current_path,
299            )),
300        ))
301    }
302
303    async fn rename(&self, from: &str, to: &str, _args: OpRename) -> Result<RpRename> {
304        let from_path = build_rooted_abs_path(&self.root, from);
305        let to_path = build_rooted_abs_path(&self.root, to);
306        match self.client.get_file_info(&to_path).await {
307            Ok(status) => {
308                if status.isdir {
309                    return Err(Error::new(ErrorKind::IsADirectory, "path should be a file")
310                        .with_context("input", &to_path));
311                } else {
312                    self.client
313                        .delete(&to_path, true)
314                        .await
315                        .map_err(parse_hdfs_error)?;
316                }
317            }
318            Err(err) => match &err {
319                HdfsError::FileNotFound(_) => {
320                    self.client
321                        .create(&to_path, WriteOptions::default().create_parent(true))
322                        .await
323                        .map_err(parse_hdfs_error)?;
324                }
325                _ => return Err(parse_hdfs_error(err)),
326            },
327        };
328
329        self.client
330            .rename(&from_path, &to_path, true)
331            .await
332            .map_err(parse_hdfs_error)?;
333
334        Ok(RpRename::default())
335    }
336}
```
