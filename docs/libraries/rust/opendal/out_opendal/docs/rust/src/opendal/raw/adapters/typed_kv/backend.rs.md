# 

opendal/raw/adapters/typed_kv/

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
19use std::vec::IntoIter;
20
21use super::Adapter;
22use super::Value;
23use crate::raw::oio::HierarchyLister;
24use crate::raw::oio::QueueBuf;
25use crate::raw::*;
26use crate::*;
27
28/// The typed kv backend which implements Accessor for typed kv adapter.
29#[derive(Debug, Clone)]
30pub struct Backend<S: Adapter> {
31    kv: Arc<S>,
32    root: String,
33    info: Arc<AccessorInfo>,
34}
35
36impl<S> Backend<S>
37where
38    S: Adapter,
39{
40    /// Create a new kv backend.
41    pub fn new(kv: S) -> Self {
42        let kv_info = kv.info();
43        Self {
44            kv: Arc::new(kv),
45            root: "/".to_string(),
46            info: {
47                let am: AccessorInfo = AccessorInfo::default();
48                am.set_root("/");
49                am.set_scheme(kv_info.scheme().into_static());
50                am.set_name(kv_info.name());
51
52                let kv_cap = kv_info.capabilities();
53                let mut cap = Capability::default();
54                if kv_cap.get {
55                    cap.read = true;
56                    cap.stat = true;
57                }
58
59                if kv_cap.set {
60                    cap.write = true;
61                    cap.write_can_empty = true;
62                }
63
64                if kv_cap.delete {
65                    cap.delete = true;
66                }
67
68                if kv_cap.scan {
69                    cap.list = true;
70                    cap.list_with_recursive = true;
71                }
72
73                if kv_cap.shared {
74                    cap.shared = true;
75                }
76
77                am.set_native_capability(cap);
78
79                am.into()
80            },
81        }
82    }
83
84    /// Configure root within this backend.
85    pub fn with_root(mut self, root: &str) -> Self {
86        let root = normalize_root(root);
87        self.info.set_root(&root);
88        self.root = root;
89        self
90    }
91}
92
93impl<S: Adapter> Access for Backend<S> {
94    type Reader = Buffer;
95    type Writer = KvWriter<S>;
96    type Lister = HierarchyLister<KvLister>;
97    type Deleter = oio::OneShotDeleter<KvDeleter<S>>;
98
99    fn info(&self) -> Arc<AccessorInfo> {
100        self.info.clone()
101    }
102
103    async fn stat(&self, path: &str, _: OpStat) -> Result<RpStat> {
104        let p = build_abs_path(&self.root, path);
105
106        if p == build_abs_path(&self.root, "") {
107            Ok(RpStat::new(Metadata::new(EntryMode::DIR)))
108        } else {
109            let bs = self.kv.get(&p).await?;
110            match bs {
111                Some(bs) => Ok(RpStat::new(bs.metadata)),
112                None => Err(Error::new(ErrorKind::NotFound, "kv doesn't have this path")),
113            }
114        }
115    }
116
117    async fn read(&self, path: &str, args: OpRead) -> Result<(RpRead, Self::Reader)> {
118        let p = build_abs_path(&self.root, path);
119
120        let bs = match self.kv.get(&p).await? {
121            // TODO: we can reuse the metadata in value to build content range.
122            Some(bs) => bs.value,
123            None => return Err(Error::new(ErrorKind::NotFound, "kv doesn't have this path")),
124        };
125
126        Ok((RpRead::new(), bs.slice(args.range().to_range_as_usize())))
127    }
128
129    async fn write(&self, path: &str, args: OpWrite) -> Result<(RpWrite, Self::Writer)> {
130        let p = build_abs_path(&self.root, path);
131
132        Ok((RpWrite::new(), KvWriter::new(self.kv.clone(), p, args)))
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
152pub struct KvLister {
153    root: String,
154    inner: IntoIter<String>,
155}
156
157impl KvLister {
158    fn new(root: &str, inner: Vec<String>) -> Self {
159        Self {
160            root: root.to_string(),
161            inner: inner.into_iter(),
162        }
163    }
164
165    fn inner_next(&mut self) -> Option<oio::Entry> {
166        self.inner.next().map(|v| {
167            let mode = if v.ends_with('/') {
168                EntryMode::DIR
169            } else {
170                EntryMode::FILE
171            };
172            let mut path = build_rel_path(&self.root, &v);
173            if path.is_empty() {
174                path = "/".to_string();
175            }
176            oio::Entry::new(&path, Metadata::new(mode))
177        })
178    }
179}
180
181impl oio::List for KvLister {
182    async fn next(&mut self) -> Result<Option<oio::Entry>> {
183        Ok(self.inner_next())
184    }
185}
186
187pub struct KvWriter<S> {
188    kv: Arc<S>,
189    path: String,
190
191    op: OpWrite,
192    buf: Option<QueueBuf>,
193    value: Option<Value>,
194}
195
196/// # Safety
197///
198/// We will only take `&mut Self` reference for KvWriter.
199unsafe impl<S: Adapter> Sync for KvWriter<S> {}
200
201impl<S> KvWriter<S> {
202    fn new(kv: Arc<S>, path: String, op: OpWrite) -> Self {
203        KvWriter {
204            kv,
205            path,
206            op,
207            buf: None,
208            value: None,
209        }
210    }
211
212    fn build(&mut self) -> Value {
213        let value = self.buf.take().map(QueueBuf::collect).unwrap_or_default();
214
215        let mut metadata = Metadata::new(EntryMode::FILE);
216        metadata.set_content_length(value.len() as u64);
217
218        if let Some(v) = self.op.cache_control() {
219            metadata.set_cache_control(v);
220        }
221        if let Some(v) = self.op.content_disposition() {
222            metadata.set_content_disposition(v);
223        }
224        if let Some(v) = self.op.content_type() {
225            metadata.set_content_type(v);
226        }
227
228        Value { metadata, value }
229    }
230}
231
232impl<S: Adapter> oio::Write for KvWriter<S> {
233    async fn write(&mut self, bs: Buffer) -> Result<()> {
234        let mut buf = self.buf.take().unwrap_or_default();
235        buf.push(bs);
236        self.buf = Some(buf);
237        Ok(())
238    }
239
240    async fn close(&mut self) -> Result<Metadata> {
241        let value = match &self.value {
242            Some(value) => value.clone(),
243            None => {
244                let value = self.build();
245                self.value = Some(value.clone());
246                value
247            }
248        };
249        let meta = value.metadata.clone();
250        self.kv.set(&self.path, value).await?;
251
252        Ok(meta)
253    }
254
255    async fn abort(&mut self) -> Result<()> {
256        self.buf = None;
257        Ok(())
258    }
259}
260
261pub struct KvDeleter<S> {
262    kv: Arc<S>,
263    root: String,
264}
265
266impl<S> KvDeleter<S> {
267    fn new(kv: Arc<S>, root: String) -> Self {
268        KvDeleter { kv, root }
269    }
270}
271
272impl<S: Adapter> oio::OneShotDelete for KvDeleter<S> {
273    async fn delete_once(&self, path: String, _: OpDelete) -> Result<()> {
274        let p = build_abs_path(&self.root, &path);
275
276        self.kv.delete(&p).await?;
277        Ok(())
278    }
279}
```
