# 

opendal/raw/tests/

write.rs

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
18use bytes::Bytes;
19use bytes::BytesMut;
20use rand::RngCore;
21use rand::thread_rng;
22use sha2::Digest;
23use sha2::Sha256;
24
25/// WriteAction represents a read action.
26#[derive(Debug, Clone, Eq, PartialEq)]
27pub enum WriteAction {
28    /// Write represents a write action with given input buf size.
29    ///
30    /// # NOTE
31    ///
32    /// The size is the input buf size, it's possible that the actual write size is smaller.
33    Write(usize),
34}
35
36/// WriteAction is used to check the correctness of the write process.
37pub struct WriteChecker {
38    chunks: Vec<Bytes>,
39    data: Bytes,
40}
41
42impl WriteChecker {
43    /// Create a new WriteChecker with given size.
44    pub fn new(size: Vec<usize>) -> Self {
45        let mut rng = thread_rng();
46
47        let mut chunks = Vec::with_capacity(size.len());
48
49        for i in size {
50            let mut bs = vec![0u8; i];
51            rng.fill_bytes(&mut bs);
52            chunks.push(Bytes::from(bs));
53        }
54
55        let data = chunks.iter().fold(BytesMut::new(), |mut acc, x| {
56            acc.extend_from_slice(x);
57            acc
58        });
59
60        WriteChecker {
61            chunks,
62            data: data.freeze(),
63        }
64    }
65
66    /// Get the check's chunks.
67    pub fn chunks(&self) -> &[Bytes] {
68        &self.chunks
69    }
70
71    /// Check the correctness of the write process.
72    pub fn check(&self, actual: &[u8]) {
73        assert_eq!(
74            format!("{:x}", Sha256::digest(actual)),
75            format!("{:x}", Sha256::digest(&self.data)),
76            "check failed: result is not expected"
77        )
78    }
79}
```
