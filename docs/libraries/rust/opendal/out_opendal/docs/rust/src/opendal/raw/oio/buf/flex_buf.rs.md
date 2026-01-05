# 

opendal/raw/oio/buf/

flex_buf.rs

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
18use bytes::Buf;
19use bytes::BufMut;
20use bytes::Bytes;
21use bytes::BytesMut;
22
23/// FlexBuf is a buffer that support frozen bytes and reuse existing allocated memory.
24///
25/// It's useful when we want to freeze the buffer and reuse the memory for the next buffer.
26pub struct FlexBuf {
27    /// Already allocated memory size of `buf`.
28    cap: usize,
29    /// Already written bytes length inside `buf`.
30    len: usize,
31    buf: BytesMut,
32    frozen: Option<Bytes>,
33}
34
35impl FlexBuf {
36    /// Initializes a new `FlexBuf` with the given capacity.
37    pub fn new(cap: usize) -> Self {
38        FlexBuf {
39            cap,
40            len: 0,
41
42            buf: BytesMut::with_capacity(cap),
43            frozen: None,
44        }
45    }
46
47    /// Put slice into flex buf.
48    ///
49    /// Return 0 means the buffer is frozen.
50    pub fn put(&mut self, bs: &[u8]) -> usize {
51        if self.frozen.is_some() {
52            return 0;
53        }
54
55        let n = (self.cap - self.len).min(bs.len());
56        self.buf.put_slice(&bs[..n]);
57        self.len += n;
58
59        if self.len >= self.cap {
60            let frozen = self.buf.split();
61            self.len = 0;
62            self.frozen = Some(frozen.freeze());
63        }
64
65        n
66    }
67
68    /// Freeze the buffer no matter it's full or not.
69    ///
70    /// It's a no-op if the buffer has already been frozen.
71    pub fn freeze(&mut self) {
72        if self.len == 0 {
73            return;
74        }
75        let frozen = self.buf.split();
76        self.len = 0;
77        self.frozen = Some(frozen.freeze());
78    }
79
80    /// Get the frozen buffer.
81    ///
82    /// Return `None` if the buffer is not frozen.
83    ///
84    /// # Notes
85    ///
86    /// This operation did nothing to the buffer. We use `&mut self` just for make
87    /// the API consistent with other APIs.
88    pub fn get(&mut self) -> Option<Bytes> {
89        self.frozen.clone()
90    }
91
92    // Advance the frozen buffer.
93    ///
94    /// # Panics
95    ///
96    /// Panic if the buffer is not frozen.
97    pub fn advance(&mut self, cnt: usize) {
98        debug_assert!(self.len == 0, "The buffer must be empty during advance");
99
100        let Some(bs) = self.frozen.as_mut() else {
101            unreachable!("It must be a bug to advance on not frozen buffer")
102        };
103        bs.advance(cnt);
104
105        if bs.is_empty() {
106            self.clean()
107        }
108    }
109
110    /// Cleanup the buffer, reset to the initial state.
111    #[inline]
112    pub fn clean(&mut self) {
113        self.frozen = None;
114        // This reserve cloud be cheap since we can reuse already allocated memory.
115        // (if all references to the frozen buffer are dropped)
116        self.buf.reserve(self.cap);
117    }
118}
```
