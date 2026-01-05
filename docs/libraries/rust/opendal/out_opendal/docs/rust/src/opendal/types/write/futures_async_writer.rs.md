# 

opendal/types/write/

futures_async_writer.rs

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
18use std::io;
19use std::pin::Pin;
20use std::task::Context;
21use std::task::Poll;
22use std::task::ready;
23
24use futures::AsyncWrite;
25use futures::SinkExt;
26
27use crate::raw::*;
28use crate::*;
29
30/// FuturesIoAsyncWriter is the adapter of [`AsyncWrite`] for [`Writer`].
31///
32/// Users can use this adapter in cases where they need to use [`AsyncWrite`] related trait.
33///
34/// FuturesIoAsyncWriter also implements [`Unpin`], [`Send`] and [`Sync`]
35pub struct FuturesAsyncWriter {
36    sink: BufferSink,
37    buf: oio::FlexBuf,
38}
39
40impl FuturesAsyncWriter {
41    /// NOTE: don't allow users to create directly.
42    #[inline]
43    pub(crate) fn new(w: WriteGenerator<oio::Writer>) -> Self {
44        FuturesAsyncWriter {
45            sink: BufferSink::new(w),
46            buf: oio::FlexBuf::new(256 * 1024),
47        }
48    }
49}
50
51impl AsyncWrite for FuturesAsyncWriter {
52    fn poll_write(
53        self: Pin<&mut Self>,
54        cx: &mut Context<'_>,
55        buf: &[u8],
56    ) -> Poll<io::Result<usize>> {
57        let this = self.get_mut();
58
59        loop {
60            let n = this.buf.put(buf);
61            if n > 0 {
62                return Poll::Ready(Ok(n));
63            }
64
65            ready!(this.sink.poll_ready_unpin(cx)).map_err(format_std_io_error)?;
66
67            let bs = this.buf.get().expect("frozen buffer must be valid");
68            this.sink
69                .start_send_unpin(Buffer::from(bs))
70                .map_err(format_std_io_error)?;
71            this.buf.clean();
72        }
73    }
74
75    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
76        let this = self.get_mut();
77
78        loop {
79            // Make sure buf has been frozen.
80            this.buf.freeze();
81            let Some(bs) = this.buf.get() else {
82                return Poll::Ready(Ok(()));
83            };
84
85            ready!(this.sink.poll_ready_unpin(cx)).map_err(format_std_io_error)?;
86            this.sink
87                .start_send_unpin(Buffer::from(bs))
88                .map_err(format_std_io_error)?;
89            this.buf.clean();
90        }
91    }
92
93    fn poll_close(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
94        let this = self.get_mut();
95
96        loop {
97            // Make sure buf has been frozen.
98            this.buf.freeze();
99            let Some(bs) = this.buf.get() else {
100                return this.sink.poll_close_unpin(cx).map_err(format_std_io_error);
101            };
102
103            ready!(this.sink.poll_ready_unpin(cx)).map_err(format_std_io_error)?;
104            this.sink
105                .start_send_unpin(Buffer::from(bs))
106                .map_err(format_std_io_error)?;
107            this.buf.clean();
108        }
109    }
110}
111
112#[cfg(test)]
113mod tests {
114    use std::sync::Arc;
115
116    use super::*;
117    use crate::raw::MaybeSend;
118
119    #[tokio::test]
120    async fn test_trait() {
121        let op = Operator::via_iter(Scheme::Memory, []).unwrap();
122
123        let acc = op.into_inner();
124        let ctx = Arc::new(WriteContext::new(
125            acc,
126            "test".to_string(),
127            OpWrite::new(),
128            OpWriter::new().with_chunk(1),
129        ));
130        let write_gen = WriteGenerator::create(ctx).await.unwrap();
131
132        let v = FuturesAsyncWriter::new(write_gen);
133
134        let _: Box<dyn Unpin + MaybeSend + Sync + 'static> = Box::new(v);
135    }
136}
```
