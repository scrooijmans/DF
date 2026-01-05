# 

opendal/raw/http_util/

body.rs

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
18use std::cmp::Ordering;
19
20use futures::Stream;
21use futures::StreamExt;
22use oio::Read;
23
24use crate::raw::*;
25use crate::*;
26
27/// The streaming body that OpenDAL's HttpClient returned.
28///
29/// We implement [`oio::Read`] for the `HttpBody`. Services can use `HttpBody` as
30/// [`Access::Read`].
31pub struct HttpBody {
32    #[cfg(not(target_arch = "wasm32"))]
33    stream: Box<dyn Stream<Item = Result<Buffer>> + Send + Sync + Unpin + 'static>,
34    #[cfg(target_arch = "wasm32")]
35    stream: Box<dyn Stream<Item = Result<Buffer>> + Unpin + 'static>,
36    size: Option<u64>,
37    consumed: u64,
38}
39
40/// # Safety
41///
42/// HttpBody is `Send` on non wasm32 targets.
43unsafe impl Send for HttpBody {}
44
45/// # Safety
46///
47/// HttpBody is sync on non wasm32 targets.
48unsafe impl Sync for HttpBody {}
49
50impl HttpBody {
51    /// Create a new `HttpBody` with given stream and optional size.
52    #[cfg(not(target_arch = "wasm32"))]
53    pub fn new<S>(stream: S, size: Option<u64>) -> Self
54    where
55        S: Stream<Item = Result<Buffer>> + Send + Sync + Unpin + 'static,
56    {
57        HttpBody {
58            stream: Box::new(stream),
59            size,
60            consumed: 0,
61        }
62    }
63
64    /// Create a new `HttpBody` with given stream and optional size.
65    #[cfg(target_arch = "wasm32")]
66    pub fn new<S>(stream: S, size: Option<u64>) -> Self
67    where
68        S: Stream<Item = Result<Buffer>> + Unpin + 'static,
69    {
70        HttpBody {
71            stream: Box::new(stream),
72            size,
73            consumed: 0,
74        }
75    }
76
77    /// Map the inner stream.
78    #[cfg(not(target_arch = "wasm32"))]
79    pub(crate) fn map_inner(
80        mut self,
81        f: impl FnOnce(
82            Box<dyn Stream<Item = Result<Buffer>> + Send + Sync + Unpin + 'static>,
83        )
84            -> Box<dyn Stream<Item = Result<Buffer>> + Send + Sync + Unpin + 'static>,
85    ) -> Self {
86        self.stream = f(self.stream);
87        self
88    }
89
90    /// Map the inner stream.
91    #[cfg(target_arch = "wasm32")]
92    pub(crate) fn map_inner(
93        mut self,
94        f: impl FnOnce(
95            Box<dyn Stream<Item = Result<Buffer>> + Unpin + 'static>,
96        ) -> Box<dyn Stream<Item = Result<Buffer>> + Unpin + 'static>,
97    ) -> Self {
98        self.stream = f(self.stream);
99        self
100    }
101
102    /// Check if the consumed data is equal to the expected content length.
103    #[inline]
104    fn check(&self) -> Result<()> {
105        let Some(expect) = self.size else {
106            return Ok(());
107        };
108
109        let actual = self.consumed;
110        match actual.cmp(&expect) {
111            Ordering::Equal => Ok(()),
112            Ordering::Less => Err(Error::new(
113                ErrorKind::Unexpected,
114                format!("http response got too little data, expect: {expect}, actual: {actual}"),
115            )
116            .set_temporary()),
117            Ordering::Greater => Err(Error::new(
118                ErrorKind::Unexpected,
119                format!("http response got too much data, expect: {expect}, actual: {actual}"),
120            )
121            .set_temporary()),
122        }
123    }
124
125    /// Read all data from the stream.
126    pub async fn to_buffer(&mut self) -> Result<Buffer> {
127        self.read_all().await
128    }
129}
130
131impl oio::Read for HttpBody {
132    async fn read(&mut self) -> Result<Buffer> {
133        match self.stream.next().await.transpose()? {
134            Some(buf) => {
135                self.consumed += buf.len() as u64;
136                Ok(buf)
137            }
138            None => {
139                self.check()?;
140                Ok(Buffer::new())
141            }
142        }
143    }
144}
```
