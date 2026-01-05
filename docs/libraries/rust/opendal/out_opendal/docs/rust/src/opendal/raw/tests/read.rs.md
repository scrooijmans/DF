# 

opendal/raw/tests/

read.rs

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
19use rand::RngCore;
20use rand::thread_rng;
21use sha2::Digest;
22use sha2::Sha256;
23
24use crate::*;
25
26/// ReadAction represents a read action.
27#[derive(Debug, Clone, Copy, Eq, PartialEq)]
28pub enum ReadAction {
29    /// Read represents a read action with given input buf size.
30    ///
31    /// # NOTE
32    ///
33    /// The size is the input buf size, it's possible that the actual read size is smaller.
34    Read(usize, usize),
35}
36
37/// ReadChecker is used to check the correctness of the read process.
38pub struct ReadChecker {
39    /// Raw Data is the data we write to the storage.
40    raw_data: Bytes,
41}
42
43impl ReadChecker {
44    /// Create a new read checker by given size and range.
45    ///
46    /// It's by design that we use a random generator to generate the raw data. The content of data
47    /// is not important, we only care about the correctness of the read process.
48    pub fn new(size: usize) -> Self {
49        let mut rng = thread_rng();
50        let mut data = vec![0; size];
51        rng.fill_bytes(&mut data);
52
53        let raw_data = Bytes::from(data);
54
55        Self { raw_data }
56    }
57
58    /// Return the raw data of this read checker.
59    pub fn data(&self) -> Bytes {
60        self.raw_data.clone()
61    }
62
63    /// check_read checks the correctness of the read process after a read action.
64    ///
65    /// - buf_size is the read action's buf size.
66    /// - output is the output of this read action.
67    fn check_read(&self, offset: usize, size: usize, output: &[u8]) {
68        if size == 0 {
69            assert_eq!(
70                output.len(),
71                0,
72                "check read failed: output must be empty if buf_size is 0"
73            );
74            return;
75        }
76
77        if size > 0 && output.is_empty() {
78            assert!(
79                offset >= self.raw_data.len(),
80                "check read failed: no data read means cur must outsides of ranged_data",
81            );
82            return;
83        }
84
85        assert!(
86            offset + output.len() <= self.raw_data.len(),
87            "check read failed: cur + output length must be less than ranged_data length, offset: {}, output: {}, ranged_data: {}",
88            offset,
89            output.len(),
90            self.raw_data.len(),
91        );
92
93        let expected = &self.raw_data[offset..offset + output.len()];
94
95        // Check the read result
96        assert_eq!(
97            format!("{:x}", Sha256::digest(output)),
98            format!("{:x}", Sha256::digest(expected)),
99            "check read failed: output bs is different with expected bs",
100        );
101    }
102
103    /// Check will check the correctness of the read process via given actions.
104    ///
105    /// Check will panic if any check failed.
106    pub async fn check(&mut self, r: Reader, actions: &[ReadAction]) {
107        for action in actions {
108            match *action {
109                ReadAction::Read(offset, size) => {
110                    let bs = r
111                        .read(offset as u64..(offset + size) as u64)
112                        .await
113                        .expect("read must success");
114                    self.check_read(offset, size, bs.to_bytes().as_ref());
115                }
116            }
117        }
118    }
119}
```
