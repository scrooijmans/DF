# 

opendal/layers/

chaos.rs

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
18use std::sync::Arc;
19use std::sync::Mutex;
20
21use rand::prelude::*;
22use rand::rngs::StdRng;
23
24use crate::raw::*;
25use crate::*;
26
27/// Inject chaos into underlying services for robustness test.
28///
29/// # Chaos
30///
31/// Chaos tests is a part of stress test. By generating errors at specified
32/// error ratio, we can reproduce underlying services error more reliable.
33///
34/// Running tests under ChaosLayer will make your application more robust.
35///
36/// For example: If we specify an error rate of 0.5, there is a 50% chance
37/// of an EOF error for every read operation.
38///
39/// # Note
40///
41/// For now, ChaosLayer only injects read operations. More operations may
42/// be added in the future.
43///
44/// # Examples
45///
46/// ```no_run
47/// # use opendal::layers::ChaosLayer;
48/// # use opendal::services;
49/// # use opendal::Operator;
50/// # use opendal::Result;
51/// # use opendal::Scheme;
52///
53/// # fn main() -> Result<()> {
54/// let _ = Operator::new(services::Memory::default())?
55///     .layer(ChaosLayer::new(0.1))
56///     .finish();
57/// Ok(())
58/// # }
59/// ```
60#[derive(Debug, Clone)]
61pub struct ChaosLayer {
62    error_ratio: f64,
63}
64
65impl ChaosLayer {
66    /// Create a new chaos layer with specified error ratio.
67    ///
68    /// # Panics
69    ///
70    /// Input error_ratio must in [0.0..=1.0]
71    pub fn new(error_ratio: f64) -> Self {
72        assert!(
73            (0.0..=1.0).contains(&error_ratio),
74            "error_ratio must between 0.0 and 1.0"
75        );
76        Self { error_ratio }
77    }
78}
79
80impl<A: Access> Layer<A> for ChaosLayer {
81    type LayeredAccess = ChaosAccessor<A>;
82
83    fn layer(&self, inner: A) -> Self::LayeredAccess {
84        ChaosAccessor {
85            inner,
86            rng: StdRng::from_entropy(),
87            error_ratio: self.error_ratio,
88        }
89    }
90}
91
92#[derive(Debug)]
93pub struct ChaosAccessor<A> {
94    inner: A,
95    rng: StdRng,
96
97    error_ratio: f64,
98}
99
100impl<A: Access> LayeredAccess for ChaosAccessor<A> {
101    type Inner = A;
102    type Reader = ChaosReader<A::Reader>;
103    type Writer = A::Writer;
104    type Lister = A::Lister;
105    type Deleter = A::Deleter;
106
107    fn inner(&self) -> &Self::Inner {
108        &self.inner
109    }
110
111    async fn read(&self, path: &str, args: OpRead) -> Result<(RpRead, Self::Reader)> {
112        self.inner
113            .read(path, args)
114            .await
115            .map(|(rp, r)| (rp, ChaosReader::new(r, self.rng.clone(), self.error_ratio)))
116    }
117
118    async fn write(&self, path: &str, args: OpWrite) -> Result<(RpWrite, Self::Writer)> {
119        self.inner.write(path, args).await
120    }
121
122    async fn list(&self, path: &str, args: OpList) -> Result<(RpList, Self::Lister)> {
123        self.inner.list(path, args).await
124    }
125
126    async fn delete(&self) -> Result<(RpDelete, Self::Deleter)> {
127        self.inner.delete().await
128    }
129}
130
131/// ChaosReader will inject error into read operations.
132pub struct ChaosReader<R> {
133    inner: R,
134    rng: Arc<Mutex<StdRng>>,
135
136    error_ratio: f64,
137}
138
139impl<R> ChaosReader<R> {
140    fn new(inner: R, rng: StdRng, error_ratio: f64) -> Self {
141        Self {
142            inner,
143            rng: Arc::new(Mutex::new(rng)),
144            error_ratio,
145        }
146    }
147
148    /// If I feel lucky, we can return the correct response. Otherwise,
149    /// we need to generate an error.
150    fn i_feel_lucky(&self) -> bool {
151        let point = self.rng.lock().unwrap().gen_range(0..=100);
152        point >= (self.error_ratio * 100.0) as i32
153    }
154
155    fn unexpected_eof() -> Error {
156        Error::new(ErrorKind::Unexpected, "I am your chaos!")
157            .with_operation("chaos")
158            .set_temporary()
159    }
160}
161
162impl<R: oio::Read> oio::Read for ChaosReader<R> {
163    async fn read(&mut self) -> Result<Buffer> {
164        if self.i_feel_lucky() {
165            self.inner.read().await
166        } else {
167            Err(Self::unexpected_eof())
168        }
169    }
170}
```
