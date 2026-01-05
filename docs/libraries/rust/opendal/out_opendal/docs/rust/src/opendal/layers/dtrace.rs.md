# 

opendal/layers/

dtrace.rs

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
18use std::ffi::CString;
19use std::fmt::Debug;
20use std::fmt::Formatter;
21
22use bytes::Buf;
23use probe::probe_lazy;
24
25use crate::raw::Access;
26use crate::raw::*;
27use crate::*;
28
29/// Support User Statically-Defined Tracing(aka USDT) on Linux
30///
31/// This layer is an experimental feature, it will be enabled by `features = ["layers-dtrace"]` in Cargo.toml.
32///
33/// For now we have following probes:
34///
35/// ### For Accessor
36///
37/// 1. ${operation}_start, arguments: path
38///     1. create_dir
39///     2. read
40///     3. write
41///     4. stat
42///     5. delete
43///     6. list
44///     7. presign
45///
46/// 2. ${operation}_end, arguments: path
47///     1. create_dir
48///     2. read
49///     3. write
50///     4. stat
51///     5. delete
52///     6. list
53///     7. presign
54///
55/// ### For Reader
56///
57/// 1. reader_read_start, arguments: path
58/// 2. reader_read_ok, arguments: path, length
59/// 3. reader_read_error, arguments: path
60///
61/// ### For Writer
62///
63/// 1. writer_write_start, arguments: path
64/// 2. writer_write_ok, arguments: path, length
65/// 3. writer_write_error, arguments: path
66/// 4. writer_abort_start, arguments: path
67/// 5. writer_abort_ok, arguments: path
68/// 6. writer_abort_error, arguments: path
69/// 7. writer_close_start, arguments: path
70/// 8. writer_close_ok, arguments: path
71/// 9. writer_close_error, arguments: path
72///
73/// Example:
74///
75/// ```no_run
76/// # use opendal::layers::DtraceLayer;
77/// # use opendal::services;
78/// # use opendal::Operator;
79/// # use opendal::Result;
80///
81/// # #[tokio::main]
82/// # async fn main() -> Result<()> {
83/// // `Accessor` provides the low level APIs, we will use `Operator` normally.
84/// let op: Operator = Operator::new(services::Fs::default().root("/tmp"))?
85///     .layer(DtraceLayer::default())
86///     .finish();
87///
88/// let path = "/tmp/test.txt";
89/// for _ in 1..100000 {
90///     let bs = vec![0; 64 * 1024 * 1024];
91///     op.write(path, bs).await?;
92///     op.read(path).await?;
93/// }
94/// Ok(())
95/// # }
96/// ```
97///
98/// Then you can use `readelf -n target/debug/examples/dtrace` to see the probes:
99///
100/// ```text
101/// Displaying notes found in: .note.stapsdt
102///   Owner                Data size        Description
103///   stapsdt              0x00000039       NT_STAPSDT (SystemTap probe descriptors)
104///     Provider: opendal
105///     Name: create_dir_start
106///     Location: 0x00000000000f8f05, Base: 0x0000000000000000, Semaphore: 0x00000000003649f8
107///     Arguments: -8@%rax
108///   stapsdt              0x00000037       NT_STAPSDT (SystemTap probe descriptors)
109///     Provider: opendal
110///     Name: create_dir_end
111///     Location: 0x00000000000f9284, Base: 0x0000000000000000, Semaphore: 0x00000000003649fa
112///     Arguments: -8@%rax
113///   stapsdt              0x0000003c       NT_STAPSDT (SystemTap probe descriptors)
114///     Provider: opendal
115///     Name: blocking_list_start
116///     Location: 0x00000000000f9487, Base: 0x0000000000000000, Semaphore: 0x0000000000364a28
117///     Arguments: -8@%rax
118///   stapsdt              0x0000003a       NT_STAPSDT (SystemTap probe descriptors)
119///     Provider: opendal
120///     Name: blocking_list_end
121///     Location: 0x00000000000f9546, Base: 0x0000000000000000, Semaphore: 0x0000000000364a2a
122///     Arguments: -8@%rax
123///   stapsdt              0x0000003c       NT_STAPSDT (SystemTap probe descriptors)
124/// ```
125#[derive(Default, Debug, Clone)]
126pub struct DtraceLayer {}
127
128impl<A: Access> Layer<A> for DtraceLayer {
129    type LayeredAccess = DTraceAccessor<A>;
130    fn layer(&self, inner: A) -> Self::LayeredAccess {
131        DTraceAccessor { inner }
132    }
133}
134
135#[derive(Clone)]
136pub struct DTraceAccessor<A: Access> {
137    inner: A,
138}
139
140impl<A: Access> Debug for DTraceAccessor<A> {
141    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
142        f.debug_struct("DTraceAccessor")
143            .field("inner", &self.inner)
144            .finish_non_exhaustive()
145    }
146}
147
148impl<A: Access> LayeredAccess for DTraceAccessor<A> {
149    type Inner = A;
150    type Reader = DtraceLayerWrapper<A::Reader>;
151    type Writer = DtraceLayerWrapper<A::Writer>;
152    type Lister = A::Lister;
153    type Deleter = A::Deleter;
154
155    fn inner(&self) -> &Self::Inner {
156        &self.inner
157    }
158
159    async fn create_dir(&self, path: &str, args: OpCreateDir) -> Result<RpCreateDir> {
160        let c_path = CString::new(path).unwrap();
161        probe_lazy!(opendal, create_dir_start, c_path.as_ptr());
162        let result = self.inner.create_dir(path, args).await;
163        probe_lazy!(opendal, create_dir_end, c_path.as_ptr());
164        result
165    }
166
167    async fn read(&self, path: &str, args: OpRead) -> Result<(RpRead, Self::Reader)> {
168        let c_path = CString::new(path).unwrap();
169        probe_lazy!(opendal, read_start, c_path.as_ptr());
170        let result = self
171            .inner
172            .read(path, args)
173            .await
174            .map(|(rp, r)| (rp, DtraceLayerWrapper::new(r, &path.to_string())));
175        probe_lazy!(opendal, read_end, c_path.as_ptr());
176        result
177    }
178
179    async fn write(&self, path: &str, args: OpWrite) -> Result<(RpWrite, Self::Writer)> {
180        let c_path = CString::new(path).unwrap();
181        probe_lazy!(opendal, write_start, c_path.as_ptr());
182        let result = self
183            .inner
184            .write(path, args)
185            .await
186            .map(|(rp, r)| (rp, DtraceLayerWrapper::new(r, &path.to_string())));
187
188        probe_lazy!(opendal, write_end, c_path.as_ptr());
189        result
190    }
191
192    async fn stat(&self, path: &str, args: OpStat) -> Result<RpStat> {
193        let c_path = CString::new(path).unwrap();
194        probe_lazy!(opendal, stat_start, c_path.as_ptr());
195        let result = self.inner.stat(path, args).await;
196        probe_lazy!(opendal, stat_end, c_path.as_ptr());
197        result
198    }
199
200    async fn delete(&self) -> Result<(RpDelete, Self::Deleter)> {
201        self.inner.delete().await
202    }
203
204    async fn list(&self, path: &str, args: OpList) -> Result<(RpList, Self::Lister)> {
205        let c_path = CString::new(path).unwrap();
206        probe_lazy!(opendal, list_start, c_path.as_ptr());
207        let result = self.inner.list(path, args).await;
208        probe_lazy!(opendal, list_end, c_path.as_ptr());
209        result
210    }
211
212    async fn presign(&self, path: &str, args: OpPresign) -> Result<RpPresign> {
213        let c_path = CString::new(path).unwrap();
214        probe_lazy!(opendal, presign_start, c_path.as_ptr());
215        let result = self.inner.presign(path, args).await;
216        probe_lazy!(opendal, presign_end, c_path.as_ptr());
217        result
218    }
219}
220
221pub struct DtraceLayerWrapper<R> {
222    inner: R,
223    path: String,
224}
225
226impl<R> DtraceLayerWrapper<R> {
227    pub fn new(inner: R, path: &String) -> Self {
228        Self {
229            inner,
230            path: path.to_string(),
231        }
232    }
233}
234
235impl<R: oio::Read> oio::Read for DtraceLayerWrapper<R> {
236    async fn read(&mut self) -> Result<Buffer> {
237        let c_path = CString::new(self.path.clone()).unwrap();
238        probe_lazy!(opendal, reader_read_start, c_path.as_ptr());
239        match self.inner.read().await {
240            Ok(bs) => {
241                probe_lazy!(opendal, reader_read_ok, c_path.as_ptr(), bs.remaining());
242                Ok(bs)
243            }
244            Err(e) => {
245                probe_lazy!(opendal, reader_read_error, c_path.as_ptr());
246                Err(e)
247            }
248        }
249    }
250}
251
252impl<R: oio::Write> oio::Write for DtraceLayerWrapper<R> {
253    async fn write(&mut self, bs: Buffer) -> Result<()> {
254        let c_path = CString::new(self.path.clone()).unwrap();
255        probe_lazy!(opendal, writer_write_start, c_path.as_ptr());
256        self.inner
257            .write(bs)
258            .await
259            .map(|_| {
260                probe_lazy!(opendal, writer_write_ok, c_path.as_ptr());
261            })
262            .inspect_err(|_| {
263                probe_lazy!(opendal, writer_write_error, c_path.as_ptr());
264            })
265    }
266
267    async fn abort(&mut self) -> Result<()> {
268        let c_path = CString::new(self.path.clone()).unwrap();
269        probe_lazy!(opendal, writer_poll_abort_start, c_path.as_ptr());
270        self.inner
271            .abort()
272            .await
273            .map(|_| {
274                probe_lazy!(opendal, writer_poll_abort_ok, c_path.as_ptr());
275            })
276            .inspect_err(|_| {
277                probe_lazy!(opendal, writer_poll_abort_error, c_path.as_ptr());
278            })
279    }
280
281    async fn close(&mut self) -> Result<Metadata> {
282        let c_path = CString::new(self.path.clone()).unwrap();
283        probe_lazy!(opendal, writer_close_start, c_path.as_ptr());
284        self.inner
285            .close()
286            .await
287            .inspect(|_| {
288                probe_lazy!(opendal, writer_close_ok, c_path.as_ptr());
289            })
290            .inspect_err(|_| {
291                probe_lazy!(opendal, writer_close_error, c_path.as_ptr());
292            })
293    }
294}
```
