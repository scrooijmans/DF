# 

opendal/types/read/

reader.rs

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
18use std::ops::Range;
19use std::ops::RangeBounds;
20use std::sync::Arc;
21
22use bytes::BufMut;
23use futures::TryStreamExt;
24
25use crate::raw::Access;
26use crate::raw::ConcurrentTasks;
27use crate::*;
28
29/// Reader is designed to read data from given path in an asynchronous
30/// manner.
31///
32/// # Usage
33///
34/// [`Reader`] provides multiple ways to read data from given reader.
35///
36/// `Reader` implements `Clone` so you can clone it and store in place where ever you want.
37///
38/// ## Direct
39///
40/// [`Reader`] provides public API including [`Reader::read`]. You can use those APIs directly without extra copy.
41///
42/// ```
43/// use opendal::Operator;
44/// use opendal::Result;
45///
46/// async fn test(op: Operator) -> Result<()> {
47///     let r = op.reader("path/to/file").await?;
48///     let bs = r.read(0..1024).await?;
49///     Ok(())
50/// }
51/// ```
52///
53/// ## Read like `Stream`
54///
55/// ```
56/// use anyhow::Result;
57/// use bytes::Bytes;
58/// use futures::TryStreamExt;
59/// use opendal::Operator;
60///
61/// async fn test(op: Operator) -> Result<()> {
62///     let s = op
63///         .reader("path/to/file")
64///         .await?
65///         .into_bytes_stream(1024..2048)
66///         .await?;
67///     let bs: Vec<Bytes> = s.try_collect().await?;
68///     Ok(())
69/// }
70/// ```
71///
72/// ## Read like `AsyncRead` and `AsyncBufRead`
73///
74/// ```
75/// use anyhow::Result;
76/// use bytes::Bytes;
77/// use futures::AsyncReadExt;
78/// use opendal::Operator;
79///
80/// async fn test(op: Operator) -> Result<()> {
81///     let mut r = op
82///         .reader("path/to/file")
83///         .await?
84///         .into_futures_async_read(1024..2048)
85///         .await?;
86///     let mut bs = vec![];
87///     let n = r.read_to_end(&mut bs).await?;
88///     Ok(())
89/// }
90/// ```
91#[derive(Clone)]
92pub struct Reader {
93    ctx: Arc<ReadContext>,
94}
95
96impl Reader {
97    /// Create a new reader.
98    ///
99    /// Create will use internal information to decide the most suitable
100    /// implementation for users.
101    ///
102    /// We don't want to expose those details to users so keep this function
103    /// in crate only.
104    pub(crate) fn new(ctx: ReadContext) -> Self {
105        Reader { ctx: Arc::new(ctx) }
106    }
107
108    /// Read give range from reader into [`Buffer`].
109    ///
110    /// This operation is zero-copy, which means it keeps the [`bytes::Bytes`] returned by underlying
111    /// storage services without any extra copy or intensive memory allocations.
112    pub async fn read(&self, range: impl RangeBounds<u64>) -> Result<Buffer> {
113        let bufs: Vec<_> = self.clone().into_stream(range).await?.try_collect().await?;
114        Ok(bufs.into_iter().flatten().collect())
115    }
116
117    /// Read all data from reader into given [`BufMut`].
118    ///
119    /// This operation will copy and write bytes into given [`BufMut`]. Allocation happens while
120    /// [`BufMut`] doesn't have enough space.
121    pub async fn read_into(
122        &self,
123        buf: &mut impl BufMut,
124        range: impl RangeBounds<u64>,
125    ) -> Result<usize> {
126        let mut stream = self.clone().into_stream(range).await?;
127
128        let mut read = 0;
129        loop {
130            let Some(bs) = stream.try_next().await? else {
131                return Ok(read);
132            };
133            read += bs.len();
134            buf.put(bs);
135        }
136    }
137
138    /// Fetch specific ranges from reader.
139    ///
140    /// This operation try to merge given ranges into a list of
141    /// non-overlapping ranges. Users may also specify a `gap` to merge
142    /// close ranges.
143    ///
144    /// The returning `Buffer` may share the same underlying memory without
145    /// any extra copy.
146    pub async fn fetch(&self, ranges: Vec<Range<u64>>) -> Result<Vec<Buffer>> {
147        let merged_ranges = self.merge_ranges(ranges.clone());
148
149        #[derive(Clone)]
150        struct FetchInput {
151            reader: Reader,
152            range: Range<u64>,
153        }
154
155        let mut tasks = ConcurrentTasks::new(
156            self.ctx.accessor().info().executor(),
157            self.ctx.options().concurrent(),
158            self.ctx.options().prefetch(),
159            |input: FetchInput| {
160                Box::pin(async move {
161                    let FetchInput { range, reader } = input.clone();
162                    (input, reader.read(range).await)
163                })
164            },
165        );
166
167        for range in merged_ranges.clone() {
168            let reader = self.clone();
169            tasks.execute(FetchInput { reader, range }).await?;
170        }
171
172        let mut merged_bufs = vec![];
173        while let Some(b) = tasks.next().await {
174            merged_bufs.push(b?);
175        }
176
177        let mut bufs = Vec::with_capacity(ranges.len());
178        for range in ranges {
179            let idx = merged_ranges.partition_point(|v| v.start <= range.start) - 1;
180            let start = range.start - merged_ranges[idx].start;
181            let end = range.end - merged_ranges[idx].start;
182            bufs.push(merged_bufs[idx].slice(start as usize..end as usize));
183        }
184
185        Ok(bufs)
186    }
187
188    /// Merge given ranges into a list of non-overlapping ranges.
189    fn merge_ranges(&self, mut ranges: Vec<Range<u64>>) -> Vec<Range<u64>> {
190        let gap = self.ctx.options().gap().unwrap_or(1024 * 1024) as u64;
191        // We don't care about the order of range with same start, they
192        // will be merged in the next step.
193        ranges.sort_unstable_by(|a, b| a.start.cmp(&b.start));
194
195        // We know that this vector will have at most element
196        let mut merged = Vec::with_capacity(ranges.len());
197        let mut cur = ranges[0].clone();
198
199        for range in ranges.into_iter().skip(1) {
200            if range.start <= cur.end + gap {
201                // There is an overlap or the gap is small enough to merge
202                cur.end = cur.end.max(range.end);
203            } else {
204                // No overlap and the gap is too large, push the current range to the list and start a new one
205                merged.push(cur);
206                cur = range;
207            }
208        }
209
210        // Push the last range
211        merged.push(cur);
212
213        merged
214    }
215
216    /// Create a buffer stream to read specific range from given reader.
217    ///
218    /// # Notes
219    ///
220    /// BufferStream is a zero-cost abstraction. It doesn't involve extra copy of data.
221    /// It will return underlying [`Buffer`] directly.
222    ///
223    /// The [`Buffer`] this stream yields can be seen as an iterator of [`Bytes`].
224    ///
225    /// # Inputs
226    ///
227    /// - `range`: The range of data to read. range like `..` it will read all data from reader.
228    ///
229    /// # Examples
230    ///
231    /// ## Basic Usage
232    ///
233    /// ```
234    /// use std::io;
235    ///
236    /// use bytes::Bytes;
237    /// use futures::TryStreamExt;
238    /// use opendal::Buffer;
239    /// use opendal::Operator;
240    /// use opendal::Result;
241    ///
242    /// async fn test(op: Operator) -> io::Result<()> {
243    ///     let mut s = op
244    ///         .reader("hello.txt")
245    ///         .await?
246    ///         .into_stream(1024..2048)
247    ///         .await?;
248    ///
249    ///     let bs: Vec<Buffer> = s.try_collect().await?;
250    ///     // We can use those buffer as bytes if we want.
251    ///     let bytes_vec: Vec<Bytes> = bs.clone().into_iter().flatten().collect();
252    ///     // Or we can merge them into a single [`Buffer`] and later use it as [`bytes::Buf`].
253    ///     let new_buffer: Buffer = bs.into_iter().flatten().collect::<Buffer>();
254    ///
255    ///     Ok(())
256    /// }
257    /// ```
258    ///
259    /// ## Concurrent Read
260    ///
261    /// The following example reads data in 256B chunks with 8 concurrent.
262    ///
263    /// ```
264    /// use std::io;
265    ///
266    /// use bytes::Bytes;
267    /// use futures::TryStreamExt;
268    /// use opendal::Buffer;
269    /// use opendal::Operator;
270    /// use opendal::Result;
271    ///
272    /// async fn test(op: Operator) -> io::Result<()> {
273    ///     let s = op
274    ///         .reader_with("hello.txt")
275    ///         .concurrent(8)
276    ///         .chunk(256)
277    ///         .await?
278    ///         .into_stream(1024..2048)
279    ///         .await?;
280    ///
281    ///     // Every buffer except the last one in the stream will be 256B.
282    ///     let bs: Vec<Buffer> = s.try_collect().await?;
283    ///     // We can use those buffer as bytes if we want.
284    ///     let bytes_vec: Vec<Bytes> = bs.clone().into_iter().flatten().collect();
285    ///     // Or we can merge them into a single [`Buffer`] and later use it as [`bytes::Buf`].
286    ///     let new_buffer: Buffer = bs.into_iter().flatten().collect::<Buffer>();
287    ///
288    ///     Ok(())
289    /// }
290    /// ```
291    pub async fn into_stream(self, range: impl RangeBounds<u64>) -> Result<BufferStream> {
292        BufferStream::create(self.ctx, range).await
293    }
294
295    /// Convert reader into [`FuturesAsyncReader`] which implements [`futures::AsyncRead`],
296    /// [`futures::AsyncSeek`] and [`futures::AsyncBufRead`].
297    ///
298    /// # Notes
299    ///
300    /// FuturesAsyncReader is not a zero-cost abstraction. The underlying reader
301    /// returns an owned [`Buffer`], which involves an extra copy operation.
302    ///
303    /// # Inputs
304    ///
305    /// - `range`: The range of data to read. range like `..` it will read all data from reader.
306    ///
307    /// # Examples
308    ///
309    /// ## Basic Usage
310    ///
311    /// ```
312    /// use std::io;
313    ///
314    /// use futures::io::AsyncReadExt;
315    /// use opendal::Operator;
316    /// use opendal::Result;
317    ///
318    /// async fn test(op: Operator) -> io::Result<()> {
319    ///     let mut r = op
320    ///         .reader("hello.txt")
321    ///         .await?
322    ///         .into_futures_async_read(1024..2048)
323    ///         .await?;
324    ///     let mut bs = Vec::new();
325    ///     r.read_to_end(&mut bs).await?;
326    ///
327    ///     Ok(())
328    /// }
329    /// ```
330    ///
331    /// ## Concurrent Read
332    ///
333    /// The following example reads data in 256B chunks with 8 concurrent.
334    ///
335    /// ```
336    /// use std::io;
337    ///
338    /// use futures::io::AsyncReadExt;
339    /// use opendal::Operator;
340    /// use opendal::Result;
341    ///
342    /// async fn test(op: Operator) -> io::Result<()> {
343    ///     let mut r = op
344    ///         .reader_with("hello.txt")
345    ///         .concurrent(8)
346    ///         .chunk(256)
347    ///         .await?
348    ///         .into_futures_async_read(1024..2048)
349    ///         .await?;
350    ///     let mut bs = Vec::new();
351    ///     r.read_to_end(&mut bs).await?;
352    ///
353    ///     Ok(())
354    /// }
355    /// ```
356    #[inline]
357    pub async fn into_futures_async_read(
358        self,
359        range: impl RangeBounds<u64>,
360    ) -> Result<FuturesAsyncReader> {
361        let range = self.ctx.parse_into_range(range).await?;
362        Ok(FuturesAsyncReader::new(self.ctx, range))
363    }
364
365    /// Convert reader into [`FuturesBytesStream`] which implements [`futures::Stream`].
366    ///
367    /// # Inputs
368    ///
369    /// - `range`: The range of data to read. range like `..` it will read all data from reader.
370    ///
371    /// # Examples
372    ///
373    /// ## Basic Usage
374    ///
375    /// ```
376    /// use std::io;
377    ///
378    /// use bytes::Bytes;
379    /// use futures::TryStreamExt;
380    /// use opendal::Operator;
381    /// use opendal::Result;
382    ///
383    /// async fn test(op: Operator) -> io::Result<()> {
384    ///     let mut s = op
385    ///         .reader("hello.txt")
386    ///         .await?
387    ///         .into_bytes_stream(1024..2048)
388    ///         .await?;
389    ///     let bs: Vec<Bytes> = s.try_collect().await?;
390    ///
391    ///     Ok(())
392    /// }
393    /// ```
394    ///
395    /// ## Concurrent Read
396    ///
397    /// The following example reads data in 256B chunks with 8 concurrent.
398    ///
399    /// ```
400    /// use std::io;
401    ///
402    /// use bytes::Bytes;
403    /// use futures::TryStreamExt;
404    /// use opendal::Operator;
405    /// use opendal::Result;
406    ///
407    /// async fn test(op: Operator) -> io::Result<()> {
408    ///     let mut s = op
409    ///         .reader_with("hello.txt")
410    ///         .concurrent(8)
411    ///         .chunk(256)
412    ///         .await?
413    ///         .into_bytes_stream(1024..2048)
414    ///         .await?;
415    ///     let bs: Vec<Bytes> = s.try_collect().await?;
416    ///
417    ///     Ok(())
418    /// }
419    /// ```
420    #[inline]
421    pub async fn into_bytes_stream(
422        self,
423        range: impl RangeBounds<u64>,
424    ) -> Result<FuturesBytesStream> {
425        FuturesBytesStream::new(self.ctx, range).await
426    }
427}
428
429#[cfg(test)]
430mod tests {
431    use bytes::Bytes;
432    use rand::Rng;
433    use rand::RngCore;
434    use rand::rngs::ThreadRng;
435
436    use super::*;
437    use crate::Operator;
438    use crate::raw::*;
439    use crate::services;
440
441    #[tokio::test]
442    async fn test_trait() -> Result<()> {
443        let op = Operator::via_iter(Scheme::Memory, [])?;
444        op.write(
445            "test",
446            Buffer::from(vec![Bytes::from("Hello"), Bytes::from("World")]),
447        )
448        .await?;
449
450        let acc = op.into_inner();
451        let ctx = ReadContext::new(acc, "test".to_string(), OpRead::new(), OpReader::new());
452
453        let _: Box<dyn Unpin + MaybeSend + Sync + 'static> = Box::new(Reader::new(ctx));
454
455        Ok(())
456    }
457
458    fn gen_random_bytes() -> Vec<u8> {
459        let mut rng = ThreadRng::default();
460        // Generate size between 1B..16MB.
461        let size = rng.gen_range(1..16 * 1024 * 1024);
462        let mut content = vec![0; size];
463        rng.fill_bytes(&mut content);
464        content
465    }
466
467    fn gen_fixed_bytes(size: usize) -> Vec<u8> {
468        let mut rng = ThreadRng::default();
469        let mut content = vec![0; size];
470        rng.fill_bytes(&mut content);
471        content
472    }
473
474    #[tokio::test]
475    async fn test_reader_read() -> Result<()> {
476        let op = Operator::via_iter(Scheme::Memory, [])?;
477        let path = "test_file";
478
479        let content = gen_random_bytes();
480        op.write(path, content.clone())
481            .await
482            .expect("write must succeed");
483
484        let reader = op.reader(path).await.unwrap();
485        let buf = reader.read(..).await.expect("read to end must succeed");
486
487        assert_eq!(buf.to_bytes(), content);
488        Ok(())
489    }
490
491    #[tokio::test]
492    async fn test_reader_read_with_chunk() -> Result<()> {
493        let op = Operator::via_iter(Scheme::Memory, [])?;
494        let path = "test_file";
495
496        let content = gen_random_bytes();
497        op.write(path, content.clone())
498            .await
499            .expect("write must succeed");
500
501        let reader = op.reader_with(path).chunk(16).await.unwrap();
502        let buf = reader.read(..).await.expect("read to end must succeed");
503
504        assert_eq!(buf.to_bytes(), content);
505        Ok(())
506    }
507
508    #[tokio::test]
509    async fn test_reader_read_with_concurrent() -> Result<()> {
510        let op = Operator::via_iter(Scheme::Memory, [])?;
511        let path = "test_file";
512
513        let content = gen_random_bytes();
514        op.write(path, content.clone())
515            .await
516            .expect("write must succeed");
517
518        let reader = op
519            .reader_with(path)
520            .chunk(128)
521            .concurrent(16)
522            .await
523            .unwrap();
524        let buf = reader.read(..).await.expect("read to end must succeed");
525
526        assert_eq!(buf.to_bytes(), content);
527        Ok(())
528    }
529
530    #[tokio::test]
531    async fn test_reader_read_into() -> Result<()> {
532        let op = Operator::via_iter(Scheme::Memory, [])?;
533        let path = "test_file";
534
535        let content = gen_random_bytes();
536        op.write(path, content.clone())
537            .await
538            .expect("write must succeed");
539
540        let reader = op.reader(path).await.unwrap();
541        let mut buf = Vec::new();
542        reader
543            .read_into(&mut buf, ..)
544            .await
545            .expect("read to end must succeed");
546
547        assert_eq!(buf, content);
548        Ok(())
549    }
550
551    #[tokio::test]
552    async fn test_merge_ranges() -> Result<()> {
553        let op = Operator::new(services::Memory::default()).unwrap().finish();
554        let path = "test_file";
555
556        let content = gen_random_bytes();
557        op.write(path, content.clone())
558            .await
559            .expect("write must succeed");
560
561        let reader = op.reader_with(path).gap(1).await.unwrap();
562
563        let ranges = vec![0..10, 10..20, 21..30, 40..50, 40..60, 45..59];
564        let merged = reader.merge_ranges(ranges.clone());
565        assert_eq!(merged, vec![0..30, 40..60]);
566        Ok(())
567    }
568
569    #[tokio::test]
570    async fn test_fetch() -> Result<()> {
571        let op = Operator::new(services::Memory::default()).unwrap().finish();
572        let path = "test_file";
573
574        let content = gen_fixed_bytes(1024);
575        op.write(path, content.clone())
576            .await
577            .expect("write must succeed");
578
579        let reader = op.reader_with(path).gap(1).await.unwrap();
580
581        let ranges = vec![
582            0..10,
583            40..50,
584            45..59,
585            10..20,
586            21..30,
587            40..50,
588            40..60,
589            45..59,
590        ];
591        let merged = reader
592            .fetch(ranges.clone())
593            .await
594            .expect("fetch must succeed");
595
596        for (i, range) in ranges.iter().enumerate() {
597            assert_eq!(
598                merged[i].to_bytes(),
599                content[range.start as usize..range.end as usize]
600            );
601        }
602        Ok(())
603    }
604}
```
