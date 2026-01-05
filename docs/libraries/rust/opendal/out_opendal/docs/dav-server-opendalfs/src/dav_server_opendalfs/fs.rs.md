# 

dav_server_opendalfs/

fs.rs

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
18use dav_server::davpath::DavPath;
19use dav_server::fs::DavMetaData;
20use dav_server::fs::FsError;
21use dav_server::fs::{DavDirEntry, FsFuture};
22use dav_server::fs::{DavFile, FsStream};
23use dav_server::fs::{DavFileSystem, ReadDirMeta};
24use futures::FutureExt;
25use futures::StreamExt;
26use opendal::Operator;
27use std::path::Path;
28
29use super::dir::OpendalStream;
30use super::file::OpendalFile;
31use super::metadata::OpendalMetaData;
32use super::utils::convert_error;
33
34/// OpendalFs is a `DavFileSystem` implementation for opendal.
35///
36/// ```
37/// use anyhow::Result;
38/// use dav_server::davpath::DavPath;
39/// use dav_server::fs::DavFileSystem;
40/// use dav_server_opendalfs::OpendalFs;
41/// use opendal::services::Memory;
42/// use opendal::Operator;
43///
44/// #[tokio::test]
45/// async fn test() -> Result<()> {
46///     let op = Operator::new(Memory::default())?.finish();
47///
48///     let webdavfs = OpendalFs::new(op);
49///
50///     let metadata = webdavfs
51///         .metadata(&DavPath::new("/").unwrap())
52///         .await
53///         .unwrap();
54///     println!("{}", metadata.is_dir());
55///
56///     Ok(())
57/// }
58/// ```
59#[derive(Clone)]
60pub struct OpendalFs {
61    pub op: Operator,
62}
63
64impl OpendalFs {
65    /// Create a new `OpendalFs` instance.
66    pub fn new(op: Operator) -> Box<OpendalFs> {
67        Box::new(OpendalFs { op })
68    }
69
70    fn fs_path(&self, path: &DavPath) -> Result<String, FsError> {
71        String::from_utf8(path.as_bytes().to_vec()).map_err(|_| FsError::GeneralFailure)
72    }
73}
74
75impl DavFileSystem for OpendalFs {
76    fn open<'a>(
77        &'a self,
78        path: &'a DavPath,
79        options: dav_server::fs::OpenOptions,
80    ) -> FsFuture<'a, Box<dyn DavFile>> {
81        async move {
82            let path = self.fs_path(path)?;
83            let file = OpendalFile::open(self.op.clone(), path, options).await?;
84            Ok(Box::new(file) as Box<dyn DavFile>)
85        }
86        .boxed()
87    }
88
89    fn read_dir<'a>(
90        &'a self,
91        path: &'a DavPath,
92        _meta: ReadDirMeta,
93    ) -> FsFuture<'a, FsStream<Box<dyn DavDirEntry>>> {
94        async move {
95            let path = self.fs_path(path)?;
96            self.op
97                .lister(path.as_str())
98                .await
99                .map(|lister| OpendalStream::new(self.op.clone(), lister, path.as_str()).boxed())
100                .map_err(convert_error)
101        }
102        .boxed()
103    }
104
105    fn metadata<'a>(&'a self, path: &'a DavPath) -> FsFuture<'a, Box<dyn DavMetaData>> {
106        async move {
107            let path = self.fs_path(path)?;
108            let opendal_metadata = self.op.stat(path.as_str()).await;
109            match opendal_metadata {
110                Ok(metadata) => {
111                    let webdav_metadata = OpendalMetaData::new(metadata);
112                    Ok(Box::new(webdav_metadata) as Box<dyn DavMetaData>)
113                }
114                Err(e) => Err(convert_error(e)),
115            }
116        }
117        .boxed()
118    }
119
120    fn create_dir<'a>(&'a self, path: &'a DavPath) -> FsFuture<'a, ()> {
121        async move {
122            let path = self.fs_path(path)?;
123
124            // check if the parent path is exist.
125            // During MKCOL processing, a server MUST make the Request-URI a member of its parent collection, unless the Request-URI is "/".  If no such ancestor exists, the method MUST fail.
126            // refer to https://datatracker.ietf.org/doc/html/rfc2518#section-8.3.1
127            let parent = Path::new(&path).parent().unwrap();
128            match self
129                .op
130                .exists(format!("{}/", parent.display()).as_str())
131                .await
132            {
133                Ok(exist) => {
134                    if !exist && parent != Path::new("/") {
135                        return Err(FsError::NotFound);
136                    }
137                }
138                Err(e) => {
139                    return Err(convert_error(e));
140                }
141            }
142
143            let path = path.as_str();
144            // check if the given path is exist (MKCOL on existing collection should fail (RFC2518:8.3.1))
145            let exist = self.op.exists(path).await;
146            match exist {
147                Ok(exist) => match exist {
148                    true => Err(FsError::Exists),
149                    false => {
150                        let res = self.op.create_dir(path).await;
151                        match res {
152                            Ok(_) => Ok(()),
153                            Err(e) => Err(convert_error(e)),
154                        }
155                    }
156                },
157                Err(e) => Err(convert_error(e)),
158            }
159        }
160        .boxed()
161    }
162
163    fn remove_dir<'a>(&'a self, path: &'a DavPath) -> FsFuture<'a, ()> {
164        self.remove_file(path)
165    }
166
167    fn remove_file<'a>(&'a self, path: &'a DavPath) -> FsFuture<'a, ()> {
168        async move {
169            let path = self.fs_path(path)?;
170            self.op.delete(&path).await.map_err(convert_error)
171        }
172        .boxed()
173    }
174
175    fn rename<'a>(&'a self, from: &'a DavPath, to: &'a DavPath) -> FsFuture<'a, ()> {
176        async move {
177            let from_path = from
178                .as_rel_ospath()
179                .to_str()
180                .ok_or(FsError::GeneralFailure)?;
181            let to_path = to.as_rel_ospath().to_str().ok_or(FsError::GeneralFailure)?;
182            if from.is_collection() {
183                let _ = self.remove_file(to).await;
184            }
185            self.op
186                .rename(from_path, to_path)
187                .await
188                .map_err(convert_error)
189        }
190        .boxed()
191    }
192
193    fn copy<'a>(&'a self, from: &'a DavPath, to: &'a DavPath) -> FsFuture<'a, ()> {
194        async move {
195            let from_path = from
196                .as_rel_ospath()
197                .to_str()
198                .ok_or(FsError::GeneralFailure)?;
199            let to_path = to.as_rel_ospath().to_str().ok_or(FsError::GeneralFailure)?;
200            self.op
201                .copy(from_path, to_path)
202                .await
203                .map_err(convert_error)
204        }
205        .boxed()
206    }
207}
```
