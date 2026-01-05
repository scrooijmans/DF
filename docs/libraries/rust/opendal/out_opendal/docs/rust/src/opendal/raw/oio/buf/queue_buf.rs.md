# 

opendal/raw/oio/buf/

queue_buf.rs

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
18use std::collections::VecDeque;
19use std::mem;
20
21use bytes::Buf;
22
23use crate::*;
24
25/// QueueBuf is a queue of [`Buffer`].
26///
27/// It's designed to allow storing multiple buffers without copying underlying bytes and consume them
28/// in order.
29///
30/// QueueBuf mainly provides the following operations:
31///
32/// - `push`: Push a new buffer in the queue.
33/// - `collect`: Collect all buffer in the queue as a new [`Buffer`]
34/// - `advance`: Advance the queue by `cnt` bytes.
35#[derive(Clone, Default)]
36pub struct QueueBuf(VecDeque<Buffer>);
37
38impl QueueBuf {
39    /// Create a new buffer queue.
40    #[inline]
41    pub fn new() -> Self {
42        Self::default()
43    }
44
45    /// Push new [`Buffer`] into the queue.
46    #[inline]
47    pub fn push(&mut self, buf: Buffer) {
48        if buf.is_empty() {
49            return;
50        }
51
52        self.0.push_back(buf);
53    }
54
55    /// Total bytes size inside the buffer queue.
56    #[inline]
57    pub fn len(&self) -> usize {
58        self.0.iter().map(|b| b.len()).sum()
59    }
60
61    /// Is the buffer queue empty.
62    #[inline]
63    pub fn is_empty(&self) -> bool {
64        self.len() == 0
65    }
66
67    /// Take the entire buffer queue and leave `self` in empty states.
68    #[inline]
69    pub fn take(&mut self) -> QueueBuf {
70        mem::take(self)
71    }
72
73    /// Build a new [`Buffer`] from the queue.
74    ///
75    /// If the queue is empty, it will return an empty buffer. Otherwise, it will iterate over all
76    /// buffers and collect them into a new buffer.
77    ///
78    /// # Notes
79    ///
80    /// There are allocation overheads when collecting multiple buffers into a new buffer. But
81    /// most of them should be acceptable since we can expect the item length of buffers are slower
82    /// than 4k.
83    #[inline]
84    pub fn collect(mut self) -> Buffer {
85        if self.0.is_empty() {
86            Buffer::new()
87        } else if self.0.len() == 1 {
88            self.0.pop_front().unwrap()
89        } else {
90            self.0.into_iter().flatten().collect()
91        }
92    }
93
94    /// Advance the buffer queue by `cnt` bytes.
95    #[inline]
96    pub fn advance(&mut self, cnt: usize) {
97        assert!(cnt <= self.len(), "cannot advance past {cnt} bytes");
98
99        let mut new_cnt = cnt;
100        while new_cnt > 0 {
101            let buf = self.0.front_mut().expect("buffer must be valid");
102            if new_cnt < buf.remaining() {
103                buf.advance(new_cnt);
104                break;
105            } else {
106                new_cnt -= buf.remaining();
107                self.0.pop_front();
108            }
109        }
110    }
111
112    /// Clear the buffer queue.
113    #[inline]
114    pub fn clear(&mut self) {
115        self.0.clear()
116    }
117}
```
