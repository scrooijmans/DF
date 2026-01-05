# 

opendal/types/write/

writer.rs

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
20use bytes::Buf;
21
22use crate::raw::*;
23use crate::*;
24
25/// Writer is designed to write data into given path in an asynchronous
26/// manner.
27///
28/// ## Notes
29///
30/// Please make sure either `close` or `abort` has been called before
31/// dropping the writer otherwise the data could be lost.
32///
33/// ## Usage
34///
35/// ### Write Multiple Chunks
36///
37/// Some services support to write multiple chunks of data into given path. Services that doesn't
38/// support write multiple chunks will return [`ErrorKind::Unsupported`] error when calling `write`
39/// at the second time.
40///
41/// ```
42/// use opendal::Operator;
43/// use opendal::Result;
44///
45/// async fn test(op: Operator) -> Result<()> {
46///     let mut w = op.writer("path/to/file").await?;
47///     w.write(vec![1; 1024]).await?;
48///     w.write(vec![2; 1024]).await?;
49///     w.close().await?;
50///     Ok(())
51/// }
52/// ```
53///
54/// ### Write like `Sink`
55///
56/// ```
57/// use anyhow::Result;
58/// use futures::SinkExt;
59/// use opendal::Operator;
60///
61/// async fn test(op: Operator) -> Result<()> {
62///     let mut w = op.writer("path/to/file").await?.into_bytes_sink();
63///     w.send(vec![1; 1024].into()).await?;
64///     w.send(vec![2; 1024].into()).await?;
65///     w.close().await?;
66///     Ok(())
67/// }
68/// ```
69///
70/// ### Write like `AsyncWrite`
71///
72/// ```
73/// use anyhow::Result;
74/// use futures::AsyncWriteExt;
75/// use opendal::Operator;
76///
77/// async fn test(op: Operator) -> Result<()> {
78///     let mut w = op.writer("path/to/file").await?.into_futures_async_write();
79///     w.write(&vec![1; 1024]).await?;
80///     w.write(&vec![2; 1024]).await?;
81///     w.close().await?;
82///     Ok(())
83/// }
84/// ```
85///
86/// ### Write with append enabled
87///
88/// Writer also supports to write with append enabled. This is useful when users want to append
89/// some data to the end of the file.
90///
91/// - If file doesn't exist, it will be created and just like calling `write`.
92/// - If file exists, data will be appended to the end of the file.
93///
94/// Possible Errors:
95///
96/// - Some services store normal file and appendable file in different way. Trying to append
97///   on non-appendable file could return [`ErrorKind::ConditionNotMatch`] error.
98/// - Services that doesn't support append will return [`ErrorKind::Unsupported`] error when
99///   creating writer with `append` enabled.
100pub struct Writer {
101    /// Keep a reference to write context in writer.
102    _ctx: Arc<WriteContext>,
103    inner: WriteGenerator<oio::Writer>,
104}
105
106impl Writer {
107    /// Create a new writer from an `oio::Writer`.
108    pub(crate) async fn new(ctx: WriteContext) -> Result<Self> {
109        let ctx = Arc::new(ctx);
110        let inner = WriteGenerator::create(ctx.clone()).await?;
111
112        Ok(Self { _ctx: ctx, inner })
113    }
114
115    /// Write [`Buffer`] into writer.
116    ///
117    /// This operation will write all data in given buffer into writer.
118    ///
119    /// ## Examples
120    ///
121    /// ```
122    /// use bytes::Bytes;
123    /// use opendal::Operator;
124    /// use opendal::Result;
125    ///
126    /// async fn test(op: Operator) -> Result<()> {
127    ///     let mut w = op.writer("hello.txt").await?;
128    ///     // Buffer can be created from continues bytes.
129    ///     w.write("hello, world").await?;
130    ///     // Buffer can also be created from non-continues bytes.
131    ///     w.write(vec![Bytes::from("hello,"), Bytes::from("world!")])
132    ///         .await?;
133    ///
134    ///     // Make sure file has been written completely.
135    ///     w.close().await?;
136    ///     Ok(())
137    /// }
138    /// ```
139    pub async fn write(&mut self, bs: impl Into<Buffer>) -> Result<()> {
140        let mut bs = bs.into();
141        while !bs.is_empty() {
142            let n = self.inner.write(bs.clone()).await?;
143            bs.advance(n);
144        }
145
146        Ok(())
147    }
148
149    /// Write [`bytes::Buf`] into inner writer.
150    ///
151    /// This operation will write all data in given buffer into writer.
152    ///
153    /// # TODO
154    ///
155    /// Optimize this function to avoid unnecessary copy.
156    pub async fn write_from(&mut self, bs: impl Buf) -> Result<()> {
157        let mut bs = bs;
158        let bs = Buffer::from(bs.copy_to_bytes(bs.remaining()));
159        self.write(bs).await
160    }
161
162    /// Abort the writer and clean up all written data.
163    ///
164    /// ## Notes
165    ///
166    /// Abort should only be called when the writer is not closed or
167    /// aborted, otherwise an unexpected error could be returned.
168    pub async fn abort(&mut self) -> Result<()> {
169        self.inner.abort().await
170    }
171
172    /// Close the writer and make sure all data have been committed.
173    ///
174    /// ## Notes
175    ///
176    /// Close should only be called when the writer is not closed or
177    /// aborted, otherwise an unexpected error could be returned.
178    pub async fn close(&mut self) -> Result<Metadata> {
179        self.inner.close().await
180    }
181
182    /// Convert writer into [`BufferSink`] which implements [`Sink<Buffer>`].
183    ///
184    /// # Notes
185    ///
186    /// BufferSink is a zero-cost abstraction. The underlying writer
187    /// will reuse the Bytes and won't perform any copy operation over data.
188    ///
189    /// # Examples
190    ///
191    /// ## Basic Usage
192    ///
193    /// ```
194    /// use std::io;
195    ///
196    /// use bytes::Bytes;
197    /// use futures::SinkExt;
198    /// use opendal::Buffer;
199    /// use opendal::Operator;
200    /// use opendal::Result;
201    ///
202    /// async fn test(op: Operator) -> io::Result<()> {
203    ///     let mut s = op.writer("hello.txt").await?.into_sink();
204    ///     let bs = "Hello, World!".as_bytes();
205    ///     s.send(Buffer::from(bs)).await?;
206    ///     s.close().await?;
207    ///
208    ///     Ok(())
209    /// }
210    /// ```
211    ///
212    /// ## Concurrent Write
213    ///
214    /// ```
215    /// use std::io;
216    ///
217    /// use bytes::Bytes;
218    /// use futures::SinkExt;
219    /// use opendal::Buffer;
220    /// use opendal::Operator;
221    /// use opendal::Result;
222    ///
223    /// async fn test(op: Operator) -> io::Result<()> {
224    ///     let mut w = op
225    ///         .writer_with("hello.txt")
226    ///         .concurrent(8)
227    ///         .chunk(256)
228    ///         .await?
229    ///         .into_sink();
230    ///     let bs = "Hello, World!".as_bytes();
231    ///     w.send(Buffer::from(bs)).await?;
232    ///     w.close().await?;
233    ///
234    ///     Ok(())
235    /// }
236    /// ```
237    pub fn into_sink(self) -> BufferSink {
238        BufferSink::new(self.inner)
239    }
240
241    /// Convert writer into [`FuturesAsyncWriter`] which implements [`futures::AsyncWrite`],
242    ///
243    /// # Notes
244    ///
245    /// FuturesAsyncWriter is not a zero-cost abstraction. The underlying writer
246    /// requires an owned [`Buffer`], which involves an extra copy operation.
247    ///
248    /// FuturesAsyncWriter is required to call `close()` to make sure all
249    /// data have been written to the storage.
250    ///
251    /// # Examples
252    ///
253    /// ## Basic Usage
254    ///
255    /// ```
256    /// use std::io;
257    ///
258    /// use futures::io::AsyncWriteExt;
259    /// use opendal::Operator;
260    /// use opendal::Result;
261    ///
262    /// async fn test(op: Operator) -> io::Result<()> {
263    ///     let mut w = op.writer("hello.txt").await?.into_futures_async_write();
264    ///     let bs = "Hello, World!".as_bytes();
265    ///     w.write_all(bs).await?;
266    ///     w.close().await?;
267    ///
268    ///     Ok(())
269    /// }
270    /// ```
271    ///
272    /// ## Concurrent Write
273    ///
274    /// ```
275    /// use std::io;
276    ///
277    /// use futures::io::AsyncWriteExt;
278    /// use opendal::Operator;
279    /// use opendal::Result;
280    ///
281    /// async fn test(op: Operator) -> io::Result<()> {
282    ///     let mut w = op
283    ///         .writer_with("hello.txt")
284    ///         .concurrent(8)
285    ///         .chunk(256)
286    ///         .await?
287    ///         .into_futures_async_write();
288    ///     let bs = "Hello, World!".as_bytes();
289    ///     w.write_all(bs).await?;
290    ///     w.close().await?;
291    ///
292    ///     Ok(())
293    /// }
294    /// ```
295    pub fn into_futures_async_write(self) -> FuturesAsyncWriter {
296        FuturesAsyncWriter::new(self.inner)
297    }
298
299    /// Convert writer into [`FuturesBytesSink`] which implements [`futures::Sink<Bytes>`].
300    ///
301    /// # Notes
302    ///
303    /// FuturesBytesSink is a zero-cost abstraction. The underlying writer
304    /// will reuse the Bytes and won't perform any copy operation.
305    ///
306    /// # Examples
307    ///
308    /// ## Basic Usage
309    ///
310    /// ```
311    /// use std::io;
312    ///
313    /// use bytes::Bytes;
314    /// use futures::SinkExt;
315    /// use opendal::Operator;
316    /// use opendal::Result;
317    ///
318    /// async fn test(op: Operator) -> io::Result<()> {
319    ///     let mut w = op.writer("hello.txt").await?.into_bytes_sink();
320    ///     let bs = "Hello, World!".as_bytes();
321    ///     w.send(Bytes::from(bs)).await?;
322    ///     w.close().await?;
323    ///
324    ///     Ok(())
325    /// }
326    /// ```
327    ///
328    /// ## Concurrent Write
329    ///
330    /// ```
331    /// use std::io;
332    ///
333    /// use bytes::Bytes;
334    /// use futures::SinkExt;
335    /// use opendal::Operator;
336    /// use opendal::Result;
337    ///
338    /// async fn test(op: Operator) -> io::Result<()> {
339    ///     let mut w = op
340    ///         .writer_with("hello.txt")
341    ///         .concurrent(8)
342    ///         .chunk(256)
343    ///         .await?
344    ///         .into_bytes_sink();
345    ///     let bs = "Hello, World!".as_bytes();
346    ///     w.send(Bytes::from(bs)).await?;
347    ///     w.close().await?;
348    ///
349    ///     Ok(())
350    /// }
351    /// ```
352    pub fn into_bytes_sink(self) -> FuturesBytesSink {
353        FuturesBytesSink::new(self.inner)
354    }
355}
356
357#[cfg(test)]
358mod tests {
359    use bytes::Bytes;
360    use rand::Rng;
361    use rand::RngCore;
362    use rand::rngs::ThreadRng;
363
364    use crate::Operator;
365    use crate::services;
366
367    fn gen_random_bytes() -> Vec<u8> {
368        let mut rng = ThreadRng::default();
369        // Generate size between 1B..16MB.
370        let size = rng.gen_range(1..16 * 1024 * 1024);
371        let mut content = vec![0; size];
372        rng.fill_bytes(&mut content);
373        content
374    }
375
376    #[tokio::test]
377    async fn test_writer_write() {
378        let op = Operator::new(services::Memory::default()).unwrap().finish();
379        let path = "test_file";
380
381        let content = gen_random_bytes();
382        let mut writer = op.writer(path).await.unwrap();
383        writer
384            .write(content.clone())
385            .await
386            .expect("write must succeed");
387        writer.close().await.expect("close must succeed");
388
389        let buf = op.read(path).await.expect("read to end mut succeed");
390
391        assert_eq!(buf.to_bytes(), content);
392    }
393
394    #[tokio::test]
395    async fn test_writer_write_from() {
396        let op = Operator::new(services::Memory::default()).unwrap().finish();
397        let path = "test_file";
398
399        let content = gen_random_bytes();
400        let mut writer = op.writer(path).await.unwrap();
401        writer
402            .write_from(Bytes::from(content.clone()))
403            .await
404            .expect("write must succeed");
405        writer.close().await.expect("close must succeed");
406
407        let buf = op.read(path).await.expect("read to end mut succeed");
408
409        assert_eq!(buf.to_bytes(), content);
410    }
411}
```
