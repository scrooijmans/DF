# 

opendal/services/ftp/

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
20use std::str;
21use std::str::FromStr;
22use std::sync::Arc;
23
24use http::Uri;
25use log::debug;
26use services::ftp::core::Manager;
27use suppaftp::FtpError;
28use suppaftp::Status;
29use suppaftp::list::File;
30use suppaftp::types::Response;
31use tokio::sync::OnceCell;
32
33use super::FTP_SCHEME;
34use super::core::FtpCore;
35use super::delete::FtpDeleter;
36use super::err::parse_error;
37use super::lister::FtpLister;
38use super::reader::FtpReader;
39use super::writer::FtpWriter;
40use crate::raw::*;
41use crate::services::FtpConfig;
42use crate::*;
43
44/// FTP and FTPS services support.
45#[doc = include_str!("docs.md")]
46#[derive(Default)]
47pub struct FtpBuilder {
48    pub(super) config: FtpConfig,
49}
50
51impl Debug for FtpBuilder {
52    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
53        f.debug_struct("FtpBuilder")
54            .field("config", &self.config)
55            .finish()
56    }
57}
58
59impl FtpBuilder {
60    /// set endpoint for ftp backend.
61    pub fn endpoint(mut self, endpoint: &str) -> Self {
62        self.config.endpoint = if endpoint.is_empty() {
63            None
64        } else {
65            Some(endpoint.to_string())
66        };
67
68        self
69    }
70
71    /// set root path for ftp backend.
72    pub fn root(mut self, root: &str) -> Self {
73        self.config.root = if root.is_empty() {
74            None
75        } else {
76            Some(root.to_string())
77        };
78
79        self
80    }
81
82    /// set user for ftp backend.
83    pub fn user(mut self, user: &str) -> Self {
84        self.config.user = if user.is_empty() {
85            None
86        } else {
87            Some(user.to_string())
88        };
89
90        self
91    }
92
93    /// set password for ftp backend.
94    pub fn password(mut self, password: &str) -> Self {
95        self.config.password = if password.is_empty() {
96            None
97        } else {
98            Some(password.to_string())
99        };
100
101        self
102    }
103}
104
105impl Builder for FtpBuilder {
106    type Config = FtpConfig;
107
108    fn build(self) -> Result<impl Access> {
109        debug!("ftp backend build started: {:?}", &self);
110        let endpoint = match &self.config.endpoint {
111            None => return Err(Error::new(ErrorKind::ConfigInvalid, "endpoint is empty")),
112            Some(v) => v,
113        };
114
115        let endpoint_uri = match endpoint.parse::<Uri>() {
116            Err(e) => {
117                return Err(Error::new(ErrorKind::ConfigInvalid, "endpoint is invalid")
118                    .with_context("endpoint", endpoint)
119                    .set_source(e));
120            }
121            Ok(uri) => uri,
122        };
123
124        let host = endpoint_uri.host().unwrap_or("127.0.0.1");
125        let port = endpoint_uri.port_u16().unwrap_or(21);
126
127        let endpoint = format!("{host}:{port}");
128
129        let enable_secure = match endpoint_uri.scheme_str() {
130            Some("ftp") => false,
131            // if the user forgot to add a scheme prefix
132            // treat it as using secured scheme
133            Some("ftps") | None => true,
134
135            Some(s) => {
136                return Err(Error::new(
137                    ErrorKind::ConfigInvalid,
138                    "endpoint is unsupported or invalid",
139                )
140                .with_context("endpoint", s));
141            }
142        };
143
144        let root = normalize_root(&self.config.root.unwrap_or_default());
145
146        let user = match &self.config.user {
147            None => "".to_string(),
148            Some(v) => v.clone(),
149        };
150
151        let password = match &self.config.password {
152            None => "".to_string(),
153            Some(v) => v.clone(),
154        };
155
156        let accessor_info = AccessorInfo::default();
157        accessor_info
158            .set_scheme(FTP_SCHEME)
159            .set_root(&root)
160            .set_native_capability(Capability {
161                stat: true,
162
163                read: true,
164
165                write: true,
166                write_can_multi: true,
167                write_can_append: true,
168
169                delete: true,
170                create_dir: true,
171
172                list: true,
173
174                shared: true,
175
176                ..Default::default()
177            });
178        let manager = Manager {
179            endpoint: endpoint.clone(),
180            root: root.clone(),
181            user: user.clone(),
182            password: password.clone(),
183            enable_secure,
184        };
185        let core = Arc::new(FtpCore {
186            info: accessor_info.into(),
187            manager,
188            pool: OnceCell::new(),
189        });
190
191        Ok(FtpBackend { core })
192    }
193}
194
195// Backend is used to serve `Accessor` support for ftp.
196#[derive(Clone)]
197pub struct FtpBackend {
198    core: Arc<FtpCore>,
199}
200
201impl Debug for FtpBackend {
202    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
203        f.debug_struct("Backend").finish()
204    }
205}
206
207impl Access for FtpBackend {
208    type Reader = FtpReader;
209    type Writer = FtpWriter;
210    type Lister = FtpLister;
211    type Deleter = oio::OneShotDeleter<FtpDeleter>;
212
213    fn info(&self) -> Arc<AccessorInfo> {
214        self.core.info.clone()
215    }
216
217    async fn create_dir(&self, path: &str, _: OpCreateDir) -> Result<RpCreateDir> {
218        let mut ftp_stream = self.core.ftp_connect(Operation::CreateDir).await?;
219
220        let paths: Vec<&str> = path.split_inclusive('/').collect();
221
222        let mut curr_path = String::new();
223
224        for path in paths {
225            curr_path.push_str(path);
226            match ftp_stream.mkdir(&curr_path).await {
227                // Do nothing if status is FileUnavailable or OK(()) is return.
228                Err(FtpError::UnexpectedResponse(Response {
229                    status: Status::FileUnavailable,
230                    ..
231                }))
232                | Ok(()) => (),
233                Err(e) => {
234                    return Err(parse_error(e));
235                }
236            }
237        }
238
239        Ok(RpCreateDir::default())
240    }
241
242    async fn stat(&self, path: &str, _: OpStat) -> Result<RpStat> {
243        let file = self.ftp_stat(path).await?;
244
245        let mode = if file.is_file() {
246            EntryMode::FILE
247        } else if file.is_directory() {
248            EntryMode::DIR
249        } else {
250            EntryMode::Unknown
251        };
252
253        let mut meta = Metadata::new(mode);
254        meta.set_content_length(file.size() as u64);
255        meta.set_last_modified(Timestamp::try_from(file.modified())?);
256
257        Ok(RpStat::new(meta))
258    }
259
260    async fn read(&self, path: &str, args: OpRead) -> Result<(RpRead, Self::Reader)> {
261        let ftp_stream = self.core.ftp_connect(Operation::Read).await?;
262
263        let reader = FtpReader::new(ftp_stream, path.to_string(), args).await?;
264        Ok((RpRead::new(), reader))
265    }
266
267    async fn write(&self, path: &str, op: OpWrite) -> Result<(RpWrite, Self::Writer)> {
268        // Ensure the parent dir exists.
269        let parent = get_parent(path);
270        let paths: Vec<&str> = parent.split('/').collect();
271
272        // TODO: we can optimize this by checking dir existence first.
273        let mut ftp_stream = self.core.ftp_connect(Operation::Write).await?;
274        let mut curr_path = String::new();
275
276        for path in paths {
277            if path.is_empty() {
278                continue;
279            }
280            curr_path.push_str(path);
281            curr_path.push('/');
282            match ftp_stream.mkdir(&curr_path).await {
283                // Do nothing if status is FileUnavailable or OK(()) is return.
284                Err(FtpError::UnexpectedResponse(Response {
285                    status: Status::FileUnavailable,
286                    ..
287                }))
288                | Ok(()) => (),
289                Err(e) => {
290                    return Err(parse_error(e));
291                }
292            }
293        }
294
295        let tmp_path = (!op.append()).then_some(build_tmp_path_of(path));
296        let w = FtpWriter::new(ftp_stream, path.to_string(), tmp_path);
297
298        Ok((RpWrite::new(), w))
299    }
300
301    async fn delete(&self) -> Result<(RpDelete, Self::Deleter)> {
302        Ok((
303            RpDelete::default(),
304            oio::OneShotDeleter::new(FtpDeleter::new(self.core.clone())),
305        ))
306    }
307
308    async fn list(&self, path: &str, _: OpList) -> Result<(RpList, Self::Lister)> {
309        let mut ftp_stream = self.core.ftp_connect(Operation::List).await?;
310
311        let pathname = if path == "/" { None } else { Some(path) };
312        let files = ftp_stream.list(pathname).await.map_err(parse_error)?;
313
314        Ok((
315            RpList::default(),
316            FtpLister::new(if path == "/" { "" } else { path }, files),
317        ))
318    }
319}
320
321impl FtpBackend {
322    pub async fn ftp_stat(&self, path: &str) -> Result<File> {
323        let mut ftp_stream = self.core.ftp_connect(Operation::Stat).await?;
324
325        let (parent, basename) = (get_parent(path), get_basename(path));
326
327        let pathname = if parent == "/" { None } else { Some(parent) };
328
329        let resp = ftp_stream.list(pathname).await.map_err(parse_error)?;
330
331        // Get stat of file.
332        let mut files = resp
333            .into_iter()
334            .filter_map(|file| File::from_str(file.as_str()).ok())
335            .filter(|f| f.name() == basename.trim_end_matches('/'))
336            .collect::<Vec<File>>();
337
338        if files.is_empty() {
339            Err(Error::new(
340                ErrorKind::NotFound,
341                "file is not found during list",
342            ))
343        } else {
344            Ok(files.remove(0))
345        }
346    }
347}
348
349#[cfg(test)]
350mod build_test {
351    use super::FtpBuilder;
352    use crate::services::FtpConfig;
353    use crate::*;
354
355    #[test]
356    fn test_build() {
357        // ftps scheme, should suffix with default port 21
358        let b = FtpBuilder::default()
359            .endpoint("ftps://ftp_server.local")
360            .build();
361        assert!(b.is_ok());
362
363        // ftp scheme
364        let b = FtpBuilder::default()
365            .endpoint("ftp://ftp_server.local:1234")
366            .build();
367        assert!(b.is_ok());
368
369        // no scheme
370        let b = FtpBuilder::default()
371            .endpoint("ftp_server.local:8765")
372            .build();
373        assert!(b.is_ok());
374
375        // invalid scheme
376        let b = FtpBuilder::default()
377            .endpoint("invalidscheme://ftp_server.local:8765")
378            .build();
379        assert!(b.is_err());
380        let e = b.unwrap_err();
381        assert_eq!(e.kind(), ErrorKind::ConfigInvalid);
382    }
383
384    #[test]
385    fn from_uri_sets_endpoint_and_root() {
386        let uri = OperatorUri::new(
387            "ftp://example.com/public/data",
388            Vec::<(String, String)>::new(),
389        )
390        .unwrap();
391
392        let cfg = FtpConfig::from_uri(&uri).unwrap();
393        assert_eq!(cfg.endpoint.as_deref(), Some("ftp://example.com"));
394        assert_eq!(cfg.root.as_deref(), Some("public/data"));
395    }
396
397    #[test]
398    fn from_uri_applies_credentials_from_query() {
399        let uri = OperatorUri::new(
400            "ftp://example.com/data",
401            vec![
402                ("user".to_string(), "alice".to_string()),
403                ("password".to_string(), "secret".to_string()),
404            ],
405        )
406        .unwrap();
407
408        let cfg = FtpConfig::from_uri(&uri).unwrap();
409        assert_eq!(cfg.endpoint.as_deref(), Some("ftp://example.com"));
410        assert_eq!(cfg.user.as_deref(), Some("alice"));
411        assert_eq!(cfg.password.as_deref(), Some("secret"));
412    }
413}
```
