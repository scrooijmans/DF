# 

opendal/raw/adapters/kv/

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
18use std::sync::Arc;
19
20use super::Adapter;
21use super::Scan;
22use crate::raw::oio::HierarchyLister;
23use crate::raw::oio::QueueBuf;
24use crate::raw::*;
25use crate::*;
26
27/// Backend of kv service. If the storage service is one k-v-like service, it should implement this kv [`Backend`] by right.
28///
29/// `Backend` implements one general logic on how to read, write, scan the data from one kv store efficiently.
30/// And the [`Adapter`] held by `Backend` will handle how to communicate with one k-v-like service really and provides
31/// a series of basic operation for this service.
32///
33/// OpenDAL developer can implement one new k-v store backend easily with help of this Backend.
34#[derive(Debug, Clone)]
35pub struct Backend<S: Adapter> {
36    kv: Arc<S>,
37    root: String,
38    info: Arc<AccessorInfo>,
39}
40
41impl<S> Backend<S>
42where
43    S: Adapter,
44{
45    /// Create a new kv backend.
46    pub fn new(kv: S) -> Self {
47        let kv_info = kv.info();
48        Self {
49            kv: Arc::new(kv),
50            root: "/".to_string(),
51            info: {
52                let am: AccessorInfo = AccessorInfo::default();
53                am.set_root("/");
54                am.set_scheme(kv_info.scheme().into_static());
55                am.set_name(kv_info.name());
56
57                let mut cap = kv_info.capabilities();
58                if cap.read {
59                    cap.stat = true;
60                }
61
62                if cap.write {
63                    cap.write_can_empty = true;
64                    cap.delete = true;
65                }
66
67                if cap.list {
68                    cap.list_with_recursive = true;
69                }
70
71                am.set_native_capability(cap);
72
73                am.into()
74            },
75        }
76    }
77
78    /// Configure root within this backend.
79    pub fn with_root(self, root: &str) -> Self {
80        self.with_normalized_root(normalize_root(root))
81    }
82
83    /// Configure root within this backend.
84    ///
85    /// This method assumes root is normalized.
86    pub(crate) fn with_normalized_root(mut self, root: String) -> Self {
87        let root = normalize_root(&root);
88        self.info.set_root(&root);
89        self.root = root;
90        self
91    }
92}
93
94impl<S: Adapter> Access for Backend<S> {
95    type Reader = Buffer;
96    type Writer = KvWriter<S>;
97    type Lister = HierarchyLister<KvLister<S::Scanner>>;
98    type Deleter = oio::OneShotDeleter<KvDeleter<S>>;
99
100    fn info(&self) -> Arc<AccessorInfo> {
101        self.info.clone()
102    }
103
104    async fn stat(&self, path: &str, _: OpStat) -> Result<RpStat> {
105        let p = build_abs_path(&self.root, path);
106
107        if p == build_abs_path(&self.root, "") {
108            Ok(RpStat::new(Metadata::new(EntryMode::DIR)))
109        } else {
110            let bs = self.kv.get(&p).await?;
111            match bs {
112                Some(bs) => Ok(RpStat::new(
113                    Metadata::new(EntryMode::FILE).with_content_length(bs.len() as u64),
114                )),
115                None => Err(Error::new(ErrorKind::NotFound, "kv doesn't have this path")),
116            }
117        }
118    }
119
120    async fn read(&self, path: &str, args: OpRead) -> Result<(RpRead, Self::Reader)> {
121        let p = build_abs_path(&self.root, path);
122        let bs = match self.kv.get(&p).await? {
123            Some(bs) => bs,
124            None => return Err(Error::new(ErrorKind::NotFound, "kv doesn't have this path")),
125        };
126        Ok((RpRead::new(), bs.slice(args.range().to_range_as_usize())))
127    }
128
129    async fn write(&self, path: &str, _: OpWrite) -> Result<(RpWrite, Self::Writer)> {
130        let p = build_abs_path(&self.root, path);
131
132        Ok((RpWrite::new(), KvWriter::new(self.kv.clone(), p)))
133    }
134
135    async fn delete(&self) -> Result<(RpDelete, Self::Deleter)> {
136        Ok((
137            RpDelete::default(),
138            oio::OneShotDeleter::new(KvDeleter::new(self.kv.clone(), self.root.clone())),
139        ))
140    }
141
142    async fn list(&self, path: &str, args: OpList) -> Result<(RpList, Self::Lister)> {
143        let p = build_abs_path(&self.root, path);
144        let res = self.kv.scan(&p).await?;
145        let lister = KvLister::new(&self.root, res);
146        let lister = HierarchyLister::new(lister, path, args.recursive());
147
148        Ok((RpList::default(), lister))
149    }
150}
151
152pub struct KvLister<Iter> {
153    root: String,
154    inner: Iter,
155}
156
157impl<Iter> KvLister<Iter>
158where
159    Iter: Scan,
160{
161    fn new(root: &str, inner: Iter) -> Self {
162        Self {
163            root: root.to_string(),
164            inner,
165        }
166    }
167
168    async fn inner_next(&mut self) -> Result<Option<oio::Entry>> {
169        Ok(self.inner.next().await?.map(|v| {
170            let mode = if v.ends_with('/') {
171                EntryMode::DIR
172            } else {
173                EntryMode::FILE
174            };
175            let mut path = build_rel_path(&self.root, &v);
176            if path.is_empty() {
177                path = "/".to_string();
178            }
179            oio::Entry::new(&path, Metadata::new(mode))
180        }))
181    }
182}
183
184impl<Iter> oio::List for KvLister<Iter>
185where
186    Iter: Scan,
187{
188    async fn next(&mut self) -> Result<Option<oio::Entry>> {
189        self.inner_next().await
190    }
191}
192
193pub struct KvWriter<S> {
194    kv: Arc<S>,
195    path: String,
196    buffer: QueueBuf,
197}
198
199impl<S> KvWriter<S> {
200    fn new(kv: Arc<S>, path: String) -> Self {
201        KvWriter {
202            kv,
203            path,
204            buffer: QueueBuf::new(),
205        }
206    }
207}
208
209/// # Safety
210///
211/// We will only take `&mut Self` reference for KvWriter.
212unsafe impl<S: Adapter> Sync for KvWriter<S> {}
213
214impl<S: Adapter> oio::Write for KvWriter<S> {
215    async fn write(&mut self, bs: Buffer) -> Result<()> {
216        self.buffer.push(bs);
217        Ok(())
218    }
219
220    async fn close(&mut self) -> Result<Metadata> {
221        let buf = self.buffer.clone().collect();
222        let length = buf.len() as u64;
223        self.kv.set(&self.path, buf).await?;
224
225        let meta = Metadata::new(EntryMode::from_path(&self.path)).with_content_length(length);
226        Ok(meta)
227    }
228
229    async fn abort(&mut self) -> Result<()> {
230        self.buffer.clear();
231        Ok(())
232    }
233}
234
235pub struct KvDeleter<S> {
236    kv: Arc<S>,
237    root: String,
238}
239
240impl<S> KvDeleter<S> {
241    fn new(kv: Arc<S>, root: String) -> Self {
242        KvDeleter { kv, root }
243    }
244}
245
246impl<S: Adapter> oio::OneShotDelete for KvDeleter<S> {
247    async fn delete_once(&self, path: String, _: OpDelete) -> Result<()> {
248        let p = build_abs_path(&self.root, &path);
249
250        self.kv.delete(&p).await?;
251        Ok(())
252    }
253}
```
