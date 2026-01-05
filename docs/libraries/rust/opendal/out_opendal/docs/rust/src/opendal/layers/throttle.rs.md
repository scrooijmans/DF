# 

opendal/layers/

throttle.rs

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
18use std::num::NonZeroU32;
19use std::sync::Arc;
20
21use governor::Quota;
22use governor::RateLimiter;
23use governor::clock::DefaultClock;
24use governor::middleware::NoOpMiddleware;
25use governor::state::InMemoryState;
26use governor::state::NotKeyed;
27
28use crate::raw::*;
29use crate::*;
30
31/// Add a bandwidth rate limiter to the underlying services.
32///
33/// # Throttle
34///
35/// There are several algorithms when it come to rate limiting techniques.
36/// This throttle layer uses Generic Cell Rate Algorithm (GCRA) provided by
37/// [Governor](https://docs.rs/governor/latest/governor/index.html).
38/// By setting the `bandwidth` and `burst`, we can control the byte flow rate of underlying services.
39///
40/// # Note
41///
42/// When setting the ThrottleLayer, always consider the largest possible operation size as the burst size,
43/// as **the burst size should be larger than any possible byte length to allow it to pass through**.
44///
45/// Read more about [Quota](https://docs.rs/governor/latest/governor/struct.Quota.html#examples)
46///
47/// # Examples
48///
49/// This example limits bandwidth to 10 KiB/s and burst size to 10 MiB.
50///
51/// ```no_run
52/// # use opendal::layers::ThrottleLayer;
53/// # use opendal::services;
54/// # use opendal::Operator;
55/// # use opendal::Result;
56/// # use opendal::Scheme;
57///
58/// # fn main() -> Result<()> {
59/// let _ = Operator::new(services::Memory::default())
60///     .expect("must init")
61///     .layer(ThrottleLayer::new(10 * 1024, 10000 * 1024))
62///     .finish();
63/// Ok(())
64/// # }
65/// ```
66#[derive(Clone)]
67pub struct ThrottleLayer {
68    bandwidth: NonZeroU32,
69    burst: NonZeroU32,
70}
71
72impl ThrottleLayer {
73    /// Create a new `ThrottleLayer` with given bandwidth and burst.
74    ///
75    /// - bandwidth: the maximum number of bytes allowed to pass through per second.
76    /// - burst: the maximum number of bytes allowed to pass through at once.
77    pub fn new(bandwidth: u32, burst: u32) -> Self {
78        assert!(bandwidth > 0);
79        assert!(burst > 0);
80        Self {
81            bandwidth: NonZeroU32::new(bandwidth).unwrap(),
82            burst: NonZeroU32::new(burst).unwrap(),
83        }
84    }
85}
86
87impl<A: Access> Layer<A> for ThrottleLayer {
88    type LayeredAccess = ThrottleAccessor<A>;
89
90    fn layer(&self, accessor: A) -> Self::LayeredAccess {
91        let rate_limiter = Arc::new(RateLimiter::direct(
92            Quota::per_second(self.bandwidth).allow_burst(self.burst),
93        ));
94        ThrottleAccessor {
95            inner: accessor,
96            rate_limiter,
97        }
98    }
99}
100
101/// Share an atomic RateLimiter instance across all threads in one operator.
102/// If want to add more observability in the future, replace the default NoOpMiddleware with other middleware types.
103/// Read more about [Middleware](https://docs.rs/governor/latest/governor/middleware/index.html)
104type SharedRateLimiter = Arc<RateLimiter<NotKeyed, InMemoryState, DefaultClock, NoOpMiddleware>>;
105
106#[derive(Debug, Clone)]
107pub struct ThrottleAccessor<A: Access> {
108    inner: A,
109    rate_limiter: SharedRateLimiter,
110}
111
112impl<A: Access> LayeredAccess for ThrottleAccessor<A> {
113    type Inner = A;
114    type Reader = ThrottleWrapper<A::Reader>;
115    type Writer = ThrottleWrapper<A::Writer>;
116    type Lister = A::Lister;
117    type Deleter = A::Deleter;
118
119    fn inner(&self) -> &Self::Inner {
120        &self.inner
121    }
122
123    async fn read(&self, path: &str, args: OpRead) -> Result<(RpRead, Self::Reader)> {
124        let limiter = self.rate_limiter.clone();
125
126        self.inner
127            .read(path, args)
128            .await
129            .map(|(rp, r)| (rp, ThrottleWrapper::new(r, limiter)))
130    }
131
132    async fn write(&self, path: &str, args: OpWrite) -> Result<(RpWrite, Self::Writer)> {
133        let limiter = self.rate_limiter.clone();
134
135        self.inner
136            .write(path, args)
137            .await
138            .map(|(rp, w)| (rp, ThrottleWrapper::new(w, limiter)))
139    }
140
141    async fn delete(&self) -> Result<(RpDelete, Self::Deleter)> {
142        self.inner.delete().await
143    }
144
145    async fn list(&self, path: &str, args: OpList) -> Result<(RpList, Self::Lister)> {
146        self.inner.list(path, args).await
147    }
148}
149
150pub struct ThrottleWrapper<R> {
151    inner: R,
152    limiter: SharedRateLimiter,
153}
154
155impl<R> ThrottleWrapper<R> {
156    pub fn new(inner: R, rate_limiter: SharedRateLimiter) -> Self {
157        Self {
158            inner,
159            limiter: rate_limiter,
160        }
161    }
162}
163
164impl<R: oio::Read> oio::Read for ThrottleWrapper<R> {
165    async fn read(&mut self) -> Result<Buffer> {
166        self.inner.read().await
167    }
168}
169
170impl<R: oio::Write> oio::Write for ThrottleWrapper<R> {
171    async fn write(&mut self, bs: Buffer) -> Result<()> {
172        let len = bs.len();
173        if len == 0 {
174            return self.inner.write(bs).await;
175        }
176
177        if len > u32::MAX as usize {
178            return Err(Error::new(
179                ErrorKind::RateLimited,
180                "request size exceeds throttle quota capacity",
181            ));
182        }
183
184        let buf_length =
185            NonZeroU32::new(len as u32).expect("len is non-zero so NonZeroU32 must exist");
186
187        self.limiter.until_n_ready(buf_length).await.map_err(|_| {
188            Error::new(
189                ErrorKind::RateLimited,
190                "burst size is smaller than the request size",
191            )
192        })?;
193
194        self.inner.write(bs).await
195    }
196
197    async fn abort(&mut self) -> Result<()> {
198        self.inner.abort().await
199    }
200
201    async fn close(&mut self) -> Result<Metadata> {
202        self.inner.close().await
203    }
204}
```
