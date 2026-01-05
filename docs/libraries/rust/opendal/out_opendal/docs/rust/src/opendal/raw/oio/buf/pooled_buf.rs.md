# 

opendal/raw/oio/buf/

pooled_buf.rs

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
19use std::fmt::Debug;
20use std::fmt::Formatter;
21use std::fmt::{self};
22use std::sync::Mutex;
23
24use bytes::BytesMut;
25
26/// PooledBuf is a buffer pool that designed for reusing already allocated bufs.
27///
28/// It works as best-effort that tries to reuse the buffer if possible. It
29/// won't block the thread if the pool is locked, just returning a new buffer
30/// or dropping existing buffer.
31pub struct PooledBuf {
32    pool: Mutex<VecDeque<BytesMut>>,
33    size: usize,
34    initial_capacity: usize,
35}
36
37impl Debug for PooledBuf {
38    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
39        f.debug_struct("PooledBuf")
40            .field("size", &self.size)
41            .field("initial_capacity", &self.initial_capacity)
42            .finish_non_exhaustive()
43    }
44}
45
46impl PooledBuf {
47    /// Create a new buffer pool with a given size.
48    pub fn new(size: usize) -> Self {
49        Self {
50            pool: Mutex::new(VecDeque::with_capacity(size)),
51            size,
52            initial_capacity: 0,
53        }
54    }
55
56    /// Set the initial capacity of the buffer.
57    ///
58    /// The default value is 0.
59    pub fn with_initial_capacity(mut self, initial_capacity: usize) -> Self {
60        self.initial_capacity = initial_capacity;
61        self
62    }
63
64    /// Get a [`BytesMut`] from the pool.
65    ///
66    /// It's guaranteed that the buffer is empty.
67    pub fn get(&self) -> BytesMut {
68        // We don't want to block the thread if the pool is locked.
69        //
70        // Just returning a new buffer in this case.
71        let Ok(mut pool) = self.pool.try_lock() else {
72            return BytesMut::with_capacity(self.initial_capacity);
73        };
74
75        if let Some(buf) = pool.pop_front() {
76            buf
77        } else {
78            BytesMut::with_capacity(self.initial_capacity)
79        }
80    }
81
82    /// Put a [`BytesMut`] back to the pool.
83    pub fn put(&self, mut buf: BytesMut) {
84        // We don't want to block the thread if the pool is locked.
85        //
86        // Just dropping the buffer in this case.
87        let Ok(mut pool) = self.pool.try_lock() else {
88            return;
89        };
90
91        if pool.len() < self.size {
92            // Clean the buffer before putting it back to the pool.
93            buf.clear();
94            pool.push_back(buf);
95        }
96    }
97}
98
99#[cfg(test)]
100mod tests {
101    use bytes::BufMut;
102
103    use super::*;
104
105    #[test]
106    fn test_pooled_buf() {
107        let pool = PooledBuf::new(2);
108
109        let mut buf1 = pool.get();
110        buf1.put_slice(b"hello, world!");
111
112        let mut buf2 = pool.get();
113        buf2.reserve(1024);
114
115        pool.put(buf1);
116        pool.put(buf2);
117
118        let buf3 = pool.get();
119        assert_eq!(buf3.len(), 0);
120        assert_eq!(buf3.capacity(), 13);
121
122        let buf4 = pool.get();
123        assert_eq!(buf4.len(), 0);
124        assert_eq!(buf4.capacity(), 1024);
125    }
126}
```
