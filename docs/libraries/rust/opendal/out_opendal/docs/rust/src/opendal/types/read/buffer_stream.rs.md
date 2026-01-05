# 

opendal/types/read/

buffer_stream.rs

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
18use std::ops::RangeBounds;
19use std::pin::Pin;
20use std::sync::Arc;
21use std::task::Context;
22use std::task::Poll;
23
24use futures::Stream;
25use futures::ready;
26
27use crate::raw::oio::Read as _;
28use crate::raw::*;
29use crate::*;
30
31/// StreamingReader will stream the content of the file without reading into
32/// memory first.
33///
34/// StreamingReader is good for small memory footprint and optimized for latency.
35pub struct StreamingReader {
36    generator: ReadGenerator,
37    reader: Option<oio::Reader>,
38}
39
40impl StreamingReader {
41    /// Create a new streaming reader.
42    #[inline]
43    fn new(ctx: Arc<ReadContext>, range: BytesRange) -> Self {
44        let generator = ReadGenerator::new(ctx.clone(), range.offset(), range.size());
45        Self {
46            generator,
47            reader: None,
48        }
49    }
50}
51
52impl oio::Read for StreamingReader {
53    async fn read(&mut self) -> Result<Buffer> {
54        loop {
55            if self.reader.is_none() {
56                self.reader = self.generator.next_reader().await?;
57            }
58            let Some(r) = self.reader.as_mut() else {
59                return Ok(Buffer::new());
60            };
61
62            let buf = r.read().await?;
63            // Reset reader to None if this reader returns empty buffer.
64            if buf.is_empty() {
65                self.reader = None;
66                continue;
67            } else {
68                return Ok(buf);
69            }
70        }
71    }
72}
73
74/// ChunkedReader will read the file in chunks.
75///
76/// ChunkedReader is good for concurrent read and optimized for throughput.
77pub struct ChunkedReader {
78    generator: ReadGenerator,
79    tasks: ConcurrentTasks<oio::Reader, Buffer>,
80    done: bool,
81}
82
83impl ChunkedReader {
84    /// Create a new chunked reader.
85    ///
86    /// # Notes
87    ///
88    /// We don't need to handle `Executor::timeout` since we are outside the layer.
89    fn new(ctx: Arc<ReadContext>, range: BytesRange) -> Self {
90        let tasks = ConcurrentTasks::new(
91            ctx.accessor().info().executor(),
92            ctx.options().concurrent(),
93            ctx.options().prefetch(),
94            |mut r: oio::Reader| {
95                Box::pin(async {
96                    match r.read_all().await {
97                        Ok(buf) => (r, Ok(buf)),
98                        Err(err) => (r, Err(err)),
99                    }
100                })
101            },
102        );
103        let generator = ReadGenerator::new(ctx, range.offset(), range.size());
104        Self {
105            generator,
106            tasks,
107            done: false,
108        }
109    }
110}
111
112impl oio::Read for ChunkedReader {
113    async fn read(&mut self) -> Result<Buffer> {
114        while self.tasks.has_remaining() && !self.done {
115            if let Some(r) = self.generator.next_reader().await? {
116                self.tasks.execute(r).await?;
117            } else {
118                self.done = true;
119                break;
120            }
121            if self.tasks.has_result() {
122                break;
123            }
124        }
125        Ok(self.tasks.next().await.transpose()?.unwrap_or_default())
126    }
127}
128
129/// BufferStream is a stream of buffers, created by [`Reader::into_stream`]
130///
131/// `BufferStream` implements `Stream` trait.
132pub struct BufferStream {
133    /// # Notes to maintainers
134    ///
135    /// The underlying reader is either a StreamingReader or a ChunkedReader.
136    ///
137    /// - If chunk is None, BufferStream will use StreamingReader to iterate
138    ///   data in streaming way.
139    /// - Otherwise, BufferStream will use ChunkedReader to read data in chunks.
140    state: State,
141}
142
143enum State {
144    Idle(Option<TwoWays<StreamingReader, ChunkedReader>>),
145    Reading(BoxedStaticFuture<(TwoWays<StreamingReader, ChunkedReader>, Result<Buffer>)>),
146}
147
148impl BufferStream {
149    /// Create a new buffer stream with already calculated offset and size.
150    pub(crate) fn new(ctx: Arc<ReadContext>, offset: u64, size: Option<u64>) -> Self {
151        debug_assert!(
152            size.is_some() || ctx.options().chunk().is_none(),
153            "size must be known if chunk is set"
154        );
155
156        let reader = if ctx.options().chunk().is_some() {
157            TwoWays::Two(ChunkedReader::new(ctx, BytesRange::new(offset, size)))
158        } else {
159            TwoWays::One(StreamingReader::new(ctx, BytesRange::new(offset, size)))
160        };
161
162        Self {
163            state: State::Idle(Some(reader)),
164        }
165    }
166
167    /// Create a new buffer stream with given range bound.
168    ///
169    /// If users is going to perform chunked read but the read size is unknown, we will parse
170    /// into range first.
171    pub(crate) async fn create(
172        ctx: Arc<ReadContext>,
173        range: impl RangeBounds<u64>,
174    ) -> Result<Self> {
175        let reader = if ctx.options().chunk().is_some() {
176            let range = ctx.parse_into_range(range).await?;
177            TwoWays::Two(ChunkedReader::new(ctx, range.into()))
178        } else {
179            TwoWays::One(StreamingReader::new(ctx, range.into()))
180        };
181
182        Ok(Self {
183            state: State::Idle(Some(reader)),
184        })
185    }
186}
187
188impl Stream for BufferStream {
189    type Item = Result<Buffer>;
190
191    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
192        let this = self.get_mut();
193        loop {
194            match &mut this.state {
195                State::Idle(reader) => {
196                    let mut reader = reader.take().unwrap();
197                    let fut = async move {
198                        let ret = reader.read().await;
199                        (reader, ret)
200                    };
201                    this.state = State::Reading(Box::pin(fut));
202                }
203                State::Reading(fut) => {
204                    let fut = fut.as_mut();
205                    let (reader, buf) = ready!(fut.poll(cx));
206                    this.state = State::Idle(Some(reader));
207                    return match buf {
208                        Ok(buf) if buf.is_empty() => Poll::Ready(None),
209                        Ok(buf) => Poll::Ready(Some(Ok(buf))),
210                        Err(err) => Poll::Ready(Some(Err(err))),
211                    };
212                }
213            }
214        }
215    }
216}
217
218#[cfg(test)]
219mod tests {
220    use std::sync::Arc;
221
222    use bytes::Buf;
223    use bytes::Bytes;
224    use futures::TryStreamExt;
225    use pretty_assertions::assert_eq;
226
227    use super::*;
228
229    #[tokio::test]
230    async fn test_trait() -> Result<()> {
231        let acc = Operator::via_iter(Scheme::Memory, [])?.into_inner();
232        let ctx = Arc::new(ReadContext::new(
233            acc,
234            "test".to_string(),
235            OpRead::new(),
236            OpReader::new(),
237        ));
238        let v = BufferStream::create(ctx, 4..8).await?;
239
240        let _: Box<dyn Unpin + MaybeSend + 'static> = Box::new(v);
241
242        Ok(())
243    }
244
245    #[tokio::test]
246    async fn test_buffer_stream() -> Result<()> {
247        let op = Operator::via_iter(Scheme::Memory, [])?;
248        op.write(
249            "test",
250            Buffer::from(vec![Bytes::from("Hello"), Bytes::from("World")]),
251        )
252        .await?;
253
254        let acc = op.into_inner();
255        let ctx = Arc::new(ReadContext::new(
256            acc,
257            "test".to_string(),
258            OpRead::new(),
259            OpReader::new(),
260        ));
261
262        let s = BufferStream::create(ctx, 4..8).await?;
263        let bufs: Vec<_> = s.try_collect().await.unwrap();
264        assert_eq!(bufs.len(), 1);
265        assert_eq!(bufs[0].chunk(), "o".as_bytes());
266
267        let buf: Buffer = bufs.into_iter().flatten().collect();
268        assert_eq!(buf.len(), 4);
269        assert_eq!(&buf.to_vec(), "oWor".as_bytes());
270
271        Ok(())
272    }
273}
```
