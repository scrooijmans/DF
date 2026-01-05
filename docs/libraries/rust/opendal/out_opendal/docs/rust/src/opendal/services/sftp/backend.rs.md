# 

opendal/services/sftp/

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
20use std::io::SeekFrom;
21use std::path::Path;
22use std::path::PathBuf;
23use std::sync::Arc;
24
25use log::debug;
26use openssh::KnownHosts;
27use tokio::io::AsyncSeekExt;
28use tokio::sync::OnceCell;
29
30use super::SFTP_SCHEME;
31use super::core::SftpCore;
32use super::delete::SftpDeleter;
33use super::error::is_not_found;
34use super::error::is_sftp_protocol_error;
35use super::error::parse_sftp_error;
36use super::lister::SftpLister;
37use super::reader::SftpReader;
38use super::writer::SftpWriter;
39use crate::raw::*;
40use crate::services::SftpConfig;
41use crate::*;
42
43/// SFTP services support. (only works on unix)
44///
45/// If you are interested in working on windows, please refer to [this](https://github.com/apache/opendal/issues/2963) issue.
46/// Welcome to leave your comments or make contributions.
47///
48/// Warning: Maximum number of file holdings is depending on the remote system configuration.
49///
50/// For example, the default value is 255 in macOS, and 1024 in linux. If you want to open
51/// lots of files, you should pay attention to close the file after using it.
52#[doc = include_str!("docs.md")]
53#[derive(Default)]
54pub struct SftpBuilder {
55    pub(super) config: SftpConfig,
56}
57
58impl Debug for SftpBuilder {
59    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
60        f.debug_struct("SftpBuilder")
61            .field("config", &self.config)
62            .finish()
63    }
64}
65
66impl SftpBuilder {
67    /// set endpoint for sftp backend.
68    /// The format is same as `openssh`, using either `[user@]hostname` or `ssh://[user@]hostname[:port]`. A username or port that is specified in the endpoint overrides the one set in the builder (but does not change the builder).
69    pub fn endpoint(mut self, endpoint: &str) -> Self {
70        self.config.endpoint = if endpoint.is_empty() {
71            None
72        } else {
73            Some(endpoint.to_string())
74        };
75
76        self
77    }
78
79    /// set root path for sftp backend.
80    /// It uses the default directory set by the remote `sftp-server` as default.
81    pub fn root(mut self, root: &str) -> Self {
82        self.config.root = if root.is_empty() {
83            None
84        } else {
85            Some(root.to_string())
86        };
87
88        self
89    }
90
91    /// set user for sftp backend.
92    pub fn user(mut self, user: &str) -> Self {
93        self.config.user = if user.is_empty() {
94            None
95        } else {
96            Some(user.to_string())
97        };
98
99        self
100    }
101
102    /// set key path for sftp backend.
103    pub fn key(mut self, key: &str) -> Self {
104        self.config.key = if key.is_empty() {
105            None
106        } else {
107            Some(key.to_string())
108        };
109
110        self
111    }
112
113    /// set known_hosts strategy for sftp backend.
114    /// available values:
115    /// - Strict (default)
116    /// - Accept
117    /// - Add
118    pub fn known_hosts_strategy(mut self, strategy: &str) -> Self {
119        self.config.known_hosts_strategy = if strategy.is_empty() {
120            None
121        } else {
122            Some(strategy.to_string())
123        };
124
125        self
126    }
127
128    /// set enable_copy for sftp backend.
129    /// It requires the server supports copy-file extension.
130    pub fn enable_copy(mut self, enable_copy: bool) -> Self {
131        self.config.enable_copy = enable_copy;
132
133        self
134    }
135}
136
137impl Builder for SftpBuilder {
138    type Config = SftpConfig;
139
140    fn build(self) -> Result<impl Access> {
141        debug!("sftp backend build started: {:?}", &self);
142        let endpoint = match self.config.endpoint.clone() {
143            Some(v) => v,
144            None => return Err(Error::new(ErrorKind::ConfigInvalid, "endpoint is empty")),
145        };
146
147        let user = self.config.user.clone();
148
149        let root = self
150            .config
151            .root
152            .clone()
153            .map(|r| normalize_root(r.as_str()))
154            .unwrap_or_default();
155
156        let known_hosts_strategy = match &self.config.known_hosts_strategy {
157            Some(v) => {
158                let v = v.to_lowercase();
159                if v == "strict" {
160                    KnownHosts::Strict
161                } else if v == "accept" {
162                    KnownHosts::Accept
163                } else if v == "add" {
164                    KnownHosts::Add
165                } else {
166                    return Err(Error::new(
167                        ErrorKind::ConfigInvalid,
168                        format!("unknown known_hosts strategy: {v}").as_str(),
169                    ));
170                }
171            }
172            None => KnownHosts::Strict,
173        };
174
175        let info = AccessorInfo::default();
176        info.set_root(root.as_str())
177            .set_scheme(SFTP_SCHEME)
178            .set_native_capability(Capability {
179                stat: true,
180
181                read: true,
182
183                write: true,
184                write_can_multi: true,
185
186                create_dir: true,
187                delete: true,
188
189                list: true,
190                list_with_limit: true,
191
192                copy: self.config.enable_copy,
193                rename: true,
194
195                shared: true,
196
197                ..Default::default()
198            });
199
200        let accessor_info = Arc::new(info);
201        let core = Arc::new(SftpCore {
202            info: accessor_info,
203            endpoint,
204            root,
205            user,
206            key: self.config.key.clone(),
207            known_hosts_strategy,
208
209            client: OnceCell::new(),
210        });
211
212        debug!("sftp backend finished: {:?}", &self);
213        Ok(SftpBackend { core })
214    }
215}
216
217/// Backend is used to serve `Accessor` support for sftp.
218#[derive(Clone)]
219pub struct SftpBackend {
220    pub core: Arc<SftpCore>,
221}
222
223impl Debug for SftpBackend {
224    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
225        f.debug_struct("SftpBackend")
226            .field("core", &self.core)
227            .finish()
228    }
229}
230
231impl Access for SftpBackend {
232    type Reader = SftpReader;
233    type Writer = SftpWriter;
234    type Lister = Option<SftpLister>;
235    type Deleter = oio::OneShotDeleter<SftpDeleter>;
236
237    fn info(&self) -> Arc<AccessorInfo> {
238        self.core.info.clone()
239    }
240
241    async fn create_dir(&self, path: &str, _: OpCreateDir) -> Result<RpCreateDir> {
242        let client = self.core.connect().await?;
243        let mut fs = client.fs();
244        fs.set_cwd(&self.core.root);
245
246        let paths = Path::new(&path).components();
247        let mut current = PathBuf::from(&self.core.root);
248        for p in paths {
249            current = current.join(p);
250            let res = fs.create_dir(p).await;
251
252            if let Err(e) = res {
253                // ignore error if dir already exists
254                if !is_sftp_protocol_error(&e) {
255                    return Err(parse_sftp_error(e));
256                }
257            }
258            fs.set_cwd(&current);
259        }
260
261        Ok(RpCreateDir::default())
262    }
263
264    async fn stat(&self, path: &str, _: OpStat) -> Result<RpStat> {
265        let client = self.core.connect().await?;
266        let mut fs = client.fs();
267        fs.set_cwd(&self.core.root);
268
269        let meta: Metadata = fs.metadata(path).await.map_err(parse_sftp_error)?.into();
270
271        Ok(RpStat::new(meta))
272    }
273
274    async fn read(&self, path: &str, args: OpRead) -> Result<(RpRead, Self::Reader)> {
275        let client = self.core.connect().await?;
276
277        let mut fs = client.fs();
278        fs.set_cwd(&self.core.root);
279
280        let path = fs.canonicalize(path).await.map_err(parse_sftp_error)?;
281
282        let mut f = client
283            .open(path.as_path())
284            .await
285            .map_err(parse_sftp_error)?;
286
287        if args.range().offset() != 0 {
288            f.seek(SeekFrom::Start(args.range().offset()))
289                .await
290                .map_err(new_std_io_error)?;
291        }
292
293        Ok((
294            RpRead::default(),
295            SftpReader::new(client, f, args.range().size()),
296        ))
297    }
298
299    async fn write(&self, path: &str, op: OpWrite) -> Result<(RpWrite, Self::Writer)> {
300        if let Some((dir, _)) = path.rsplit_once('/') {
301            self.create_dir(dir, OpCreateDir::default()).await?;
302        }
303
304        let client = self.core.connect().await?;
305
306        let mut fs = client.fs();
307        fs.set_cwd(&self.core.root);
308        let path = fs.canonicalize(path).await.map_err(parse_sftp_error)?;
309
310        let mut option = client.options();
311        option.create(true);
312        if op.append() {
313            option.append(true);
314        } else {
315            option.write(true).truncate(true);
316        }
317
318        let file = option.open(path).await.map_err(parse_sftp_error)?;
319
320        Ok((RpWrite::new(), SftpWriter::new(file)))
321    }
322
323    async fn delete(&self) -> Result<(RpDelete, Self::Deleter)> {
324        Ok((
325            RpDelete::default(),
326            oio::OneShotDeleter::new(SftpDeleter::new(self.core.clone())),
327        ))
328    }
329
330    async fn list(&self, path: &str, _: OpList) -> Result<(RpList, Self::Lister)> {
331        let client = self.core.connect().await?;
332        let mut fs = client.fs();
333        fs.set_cwd(&self.core.root);
334
335        let file_path = format!("./{path}");
336
337        let dir = match fs.open_dir(&file_path).await {
338            Ok(dir) => dir,
339            Err(e) => {
340                if is_not_found(&e) {
341                    return Ok((RpList::default(), None));
342                } else {
343                    return Err(parse_sftp_error(e));
344                }
345            }
346        }
347        .read_dir();
348
349        Ok((
350            RpList::default(),
351            Some(SftpLister::new(dir, path.to_owned())),
352        ))
353    }
354
355    async fn copy(&self, from: &str, to: &str, _: OpCopy) -> Result<RpCopy> {
356        let client = self.core.connect().await?;
357
358        let mut fs = client.fs();
359        fs.set_cwd(&self.core.root);
360
361        if let Some((dir, _)) = to.rsplit_once('/') {
362            self.create_dir(dir, OpCreateDir::default()).await?;
363        }
364
365        let src = fs.canonicalize(from).await.map_err(parse_sftp_error)?;
366        let dst = fs.canonicalize(to).await.map_err(parse_sftp_error)?;
367        let mut src_file = client.open(&src).await.map_err(parse_sftp_error)?;
368        let mut dst_file = client.create(dst).await.map_err(parse_sftp_error)?;
369
370        src_file
371            .copy_all_to(&mut dst_file)
372            .await
373            .map_err(parse_sftp_error)?;
374
375        Ok(RpCopy::default())
376    }
377
378    async fn rename(&self, from: &str, to: &str, _: OpRename) -> Result<RpRename> {
379        let client = self.core.connect().await?;
380
381        let mut fs = client.fs();
382        fs.set_cwd(&self.core.root);
383
384        if let Some((dir, _)) = to.rsplit_once('/') {
385            self.create_dir(dir, OpCreateDir::default()).await?;
386        }
387        fs.rename(from, to).await.map_err(parse_sftp_error)?;
388
389        Ok(RpRename::default())
390    }
391}
```
