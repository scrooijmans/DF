# 

opendal/raw/oio/write/

position_write.rs

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
20use futures::Future;
21use futures::FutureExt;
22use futures::select;
23
24use crate::raw::*;
25use crate::*;
26
27/// PositionWrite is used to implement [`oio::Write`] based on position write.
28///
29/// # Services
30///
31/// Services like fs support position write.
32///
33/// # Architecture
34///
35/// The architecture after adopting [`PositionWrite`]:
36///
37/// - Services impl `PositionWrite`
38/// - `PositionWriter` impl `Write`
39/// - Expose `PositionWriter` as `Accessor::Writer`
40///
41/// # Requirements
42///
43/// Services that implement `PositionWrite` must fulfill the following requirements:
44///
45/// - Writing data based on position: `offset`.
46pub trait PositionWrite: Send + Sync + Unpin + 'static {
47    /// write_all_at is used to write the data to underlying storage at the specified offset.
48    fn write_all_at(
49        &self,
50        offset: u64,
51        buf: Buffer,
52    ) -> impl Future<Output = Result<()>> + MaybeSend;
53
54    /// close is used to close the underlying file.
55    fn close(&self) -> impl Future<Output = Result<Metadata>> + MaybeSend;
56
57    /// abort is used to abort the underlying abort.
58    fn abort(&self) -> impl Future<Output = Result<()>> + MaybeSend;
59}
60
61struct WriteInput<W: PositionWrite> {
62    w: Arc<W>,
63    executor: Executor,
64
65    offset: u64,
66    bytes: Buffer,
67}
68
69/// PositionWriter will implement [`oio::Write`] based on position write.
70pub struct PositionWriter<W: PositionWrite> {
71    w: Arc<W>,
72    executor: Executor,
73
74    next_offset: u64,
75    cache: Option<Buffer>,
76    tasks: ConcurrentTasks<WriteInput<W>, ()>,
77}
78
79#[allow(dead_code)]
80impl<W: PositionWrite> PositionWriter<W> {
81    /// Create a new PositionWriter.
82    pub fn new(info: Arc<AccessorInfo>, inner: W, concurrent: usize) -> Self {
83        let executor = info.executor();
84
85        Self {
86            w: Arc::new(inner),
87            executor: executor.clone(),
88            next_offset: 0,
89            cache: None,
90
91            tasks: ConcurrentTasks::new(executor, concurrent, 8192, |input| {
92                Box::pin(async move {
93                    let fut = input.w.write_all_at(input.offset, input.bytes.clone());
94                    match input.executor.timeout() {
95                        None => {
96                            let result = fut.await;
97                            (input, result)
98                        }
99                        Some(timeout) => {
100                            let result = select! {
101                                result = fut.fuse() => {
102                                    result
103                                }
104                                _ = timeout.fuse() => {
105                                      Err(Error::new(
106                                            ErrorKind::Unexpected, "write position timeout")
107                                                .with_context("offset", input.offset.to_string())
108                                                .set_temporary())
109                                }
110                            };
111                            (input, result)
112                        }
113                    }
114                })
115            }),
116        }
117    }
118
119    fn fill_cache(&mut self, bs: Buffer) -> usize {
120        let size = bs.len();
121        assert!(self.cache.is_none());
122        self.cache = Some(bs);
123        size
124    }
125}
126
127impl<W: PositionWrite> oio::Write for PositionWriter<W> {
128    async fn write(&mut self, bs: Buffer) -> Result<()> {
129        if self.cache.is_none() {
130            let _ = self.fill_cache(bs);
131            return Ok(());
132        }
133
134        let bytes = self.cache.clone().expect("pending write must exist");
135        let length = bytes.len() as u64;
136        let offset = self.next_offset;
137
138        self.tasks
139            .execute(WriteInput {
140                w: self.w.clone(),
141                executor: self.executor.clone(),
142                offset,
143                bytes,
144            })
145            .await?;
146        self.cache = None;
147        self.next_offset += length;
148        let _ = self.fill_cache(bs);
149        Ok(())
150    }
151
152    async fn close(&mut self) -> Result<Metadata> {
153        // Make sure all tasks are finished.
154        while self.tasks.next().await.transpose()?.is_some() {}
155
156        if let Some(buffer) = self.cache.clone() {
157            let offset = self.next_offset;
158            self.w.write_all_at(offset, buffer).await?;
159            self.cache = None;
160        }
161        self.w.close().await
162    }
163
164    async fn abort(&mut self) -> Result<()> {
165        self.tasks.clear();
166        self.cache = None;
167        self.w.abort().await?;
168        Ok(())
169    }
170}
171
172#[cfg(test)]
173mod tests {
174    use std::collections::HashSet;
175    use std::sync::Mutex;
176    use std::time::Duration;
177
178    use pretty_assertions::assert_eq;
179    use rand::Rng;
180    use rand::RngCore;
181    use rand::thread_rng;
182    use tokio::time::sleep;
183
184    use super::*;
185    use crate::raw::oio::Write;
186
187    struct TestWrite {
188        length: u64,
189        bytes: HashSet<u64>,
190    }
191
192    impl TestWrite {
193        pub fn new() -> Arc<Mutex<Self>> {
194            let v = Self {
195                bytes: HashSet::new(),
196                length: 0,
197            };
198
199            Arc::new(Mutex::new(v))
200        }
201    }
202
203    impl PositionWrite for Arc<Mutex<TestWrite>> {
204        async fn write_all_at(&self, offset: u64, buf: Buffer) -> Result<()> {
205            // Add an async sleep here to enforce some pending.
206            sleep(Duration::from_millis(50)).await;
207
208            // We will have 10% percent rate for write part to fail.
209            if thread_rng().gen_bool(1.0 / 10.0) {
210                return Err(
211                    Error::new(ErrorKind::Unexpected, "I'm a crazy monkey!").set_temporary()
212                );
213            }
214
215            let mut test = self.lock().unwrap();
216            let size = buf.len() as u64;
217            test.length += size;
218
219            let input = (offset..offset + size).collect::<HashSet<_>>();
220
221            assert!(
222                test.bytes.is_disjoint(&input),
223                "input should not have overlap"
224            );
225            test.bytes.extend(input);
226
227            Ok(())
228        }
229
230        async fn close(&self) -> Result<Metadata> {
231            Ok(Metadata::default())
232        }
233
234        async fn abort(&self) -> Result<()> {
235            Ok(())
236        }
237    }
238
239    #[tokio::test]
240    async fn test_position_writer_with_concurrent_errors() {
241        let mut rng = thread_rng();
242
243        let mut w = PositionWriter::new(Arc::default(), TestWrite::new(), 200);
244        let mut total_size = 0u64;
245
246        for _ in 0..1000 {
247            let size = rng.gen_range(1..1024);
248            total_size += size as u64;
249
250            let mut bs = vec![0; size];
251            rng.fill_bytes(&mut bs);
252
253            loop {
254                match w.write(bs.clone().into()).await {
255                    Ok(_) => break,
256                    Err(e) => {
257                        println!("write error: {e:?}");
258                        continue;
259                    }
260                }
261            }
262        }
263
264        loop {
265            match w.close().await {
266                Ok(n) => {
267                    println!("close: {n:?}");
268                    break;
269                }
270                Err(e) => {
271                    println!("close error: {e:?}");
272                    continue;
273                }
274            }
275        }
276
277        let actual_bytes = w.w.lock().unwrap().bytes.clone();
278        let expected_bytes: HashSet<_> = (0..total_size).collect();
279        assert_eq!(actual_bytes, expected_bytes);
280
281        let actual_size = w.w.lock().unwrap().length;
282        assert_eq!(actual_size, total_size);
283    }
284}
```
