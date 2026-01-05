# 

opendal/raw/

atomic_util.rs

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
18use std::sync::atomic::AtomicU64;
19use std::sync::atomic::Ordering;
20
21/// AtomicContentLength is a wrapper of AtomicU64 that used to store content length.
22///
23/// It provides a way to store and load content length in atomic way, so caller don't need to
24/// use `Mutex` or `RwLock` to protect the content length.
25///
26/// We use value `u64::MAX` to represent unknown size, it's impossible for us to
27/// handle a file that has `u64::MAX` bytes.
28#[derive(Debug)]
29pub struct AtomicContentLength(AtomicU64);
30
31impl Default for AtomicContentLength {
32    fn default() -> Self {
33        Self::new()
34    }
35}
36
37impl AtomicContentLength {
38    /// Create a new AtomicContentLength.
39    pub const fn new() -> Self {
40        Self(AtomicU64::new(u64::MAX))
41    }
42
43    /// Load content length from AtomicU64.
44    #[inline]
45    pub fn load(&self) -> Option<u64> {
46        match self.0.load(Ordering::Relaxed) {
47            u64::MAX => None,
48            v => Some(v),
49        }
50    }
51
52    /// Store content length to AtomicU64.
53    #[inline]
54    pub fn store(&self, v: u64) {
55        self.0.store(v, Ordering::Relaxed)
56    }
57}
```
