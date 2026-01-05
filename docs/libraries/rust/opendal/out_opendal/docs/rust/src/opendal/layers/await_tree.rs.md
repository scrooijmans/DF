# 

opendal/layers/

await_tree.rs

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
18use await_tree::InstrumentAwait;
19use futures::Future;
20
21use crate::raw::*;
22use crate::*;
23
24/// Add an Instrument await-tree for actor-based applications to the underlying services.
25///
26/// # AwaitTree
27///
28/// await-tree allows developers to dump this execution tree at runtime,
29/// with the span of each Future annotated by instrument_await.
30/// Read more about [await-tree](https://docs.rs/await-tree/latest/await_tree/)
31///
32/// # Examples
33///
34/// ```no_run
35/// # use opendal::layers::AwaitTreeLayer;
36/// # use opendal::services;
37/// # use opendal::Operator;
38/// # use opendal::Result;
39/// # use opendal::Scheme;
40///
41/// # fn main() -> Result<()> {
42/// let _ = Operator::new(services::Memory::default())?
43///     .layer(AwaitTreeLayer::new())
44///     .finish();
45/// Ok(())
46/// # }
47/// ```
48#[derive(Clone, Default)]
49pub struct AwaitTreeLayer {}
50
51impl AwaitTreeLayer {
52    /// Create a new `AwaitTreeLayer`.
53    pub fn new() -> Self {
54        Self {}
55    }
56}
57
58impl<A: Access> Layer<A> for AwaitTreeLayer {
59    type LayeredAccess = AwaitTreeAccessor<A>;
60
61    fn layer(&self, accessor: A) -> Self::LayeredAccess {
62        AwaitTreeAccessor { inner: accessor }
63    }
64}
65
66#[derive(Debug, Clone)]
67pub struct AwaitTreeAccessor<A: Access> {
68    inner: A,
69}
70
71impl<A: Access> LayeredAccess for AwaitTreeAccessor<A> {
72    type Inner = A;
73    type Reader = AwaitTreeWrapper<A::Reader>;
74    type Writer = AwaitTreeWrapper<A::Writer>;
75    type Lister = AwaitTreeWrapper<A::Lister>;
76    type Deleter = AwaitTreeWrapper<A::Deleter>;
77
78    fn inner(&self) -> &Self::Inner {
79        &self.inner
80    }
81
82    async fn read(&self, path: &str, args: OpRead) -> Result<(RpRead, Self::Reader)> {
83        self.inner
84            .read(path, args)
85            .instrument_await(format!("opendal::{}", Operation::Read))
86            .await
87            .map(|(rp, r)| (rp, AwaitTreeWrapper::new(r)))
88    }
89
90    async fn write(&self, path: &str, args: OpWrite) -> Result<(RpWrite, Self::Writer)> {
91        self.inner
92            .write(path, args)
93            .instrument_await(format!("opendal::{}", Operation::Write))
94            .await
95            .map(|(rp, r)| (rp, AwaitTreeWrapper::new(r)))
96    }
97
98    async fn copy(&self, from: &str, to: &str, args: OpCopy) -> Result<RpCopy> {
99        self.inner()
100            .copy(from, to, args)
101            .instrument_await(format!("opendal::{}", Operation::Copy))
102            .await
103    }
104
105    async fn rename(&self, from: &str, to: &str, args: OpRename) -> Result<RpRename> {
106        self.inner()
107            .rename(from, to, args)
108            .instrument_await(format!("opendal::{}", Operation::Rename))
109            .await
110    }
111
112    async fn stat(&self, path: &str, args: OpStat) -> Result<RpStat> {
113        self.inner
114            .stat(path, args)
115            .instrument_await(format!("opendal::{}", Operation::Stat))
116            .await
117    }
118
119    async fn delete(&self) -> Result<(RpDelete, Self::Deleter)> {
120        self.inner
121            .delete()
122            .instrument_await(format!("opendal::{}", Operation::Delete))
123            .await
124            .map(|(rp, r)| (rp, AwaitTreeWrapper::new(r)))
125    }
126
127    async fn list(&self, path: &str, args: OpList) -> Result<(RpList, Self::Lister)> {
128        self.inner
129            .list(path, args)
130            .instrument_await(format!("opendal::{}", Operation::List))
131            .await
132            .map(|(rp, r)| (rp, AwaitTreeWrapper::new(r)))
133    }
134
135    async fn presign(&self, path: &str, args: OpPresign) -> Result<RpPresign> {
136        self.inner
137            .presign(path, args)
138            .instrument_await(format!("opendal::{}", Operation::Presign))
139            .await
140    }
141}
142
143pub struct AwaitTreeWrapper<R> {
144    inner: R,
145}
146
147impl<R> AwaitTreeWrapper<R> {
148    fn new(inner: R) -> Self {
149        Self { inner }
150    }
151}
152
153impl<R: oio::Read> oio::Read for AwaitTreeWrapper<R> {
154    async fn read(&mut self) -> Result<Buffer> {
155        self.inner
156            .read()
157            .instrument_await(format!("opendal::{}", Operation::Read))
158            .await
159    }
160}
161
162impl<R: oio::Write> oio::Write for AwaitTreeWrapper<R> {
163    fn write(&mut self, bs: Buffer) -> impl Future<Output = Result<()>> + MaybeSend {
164        self.inner
165            .write(bs)
166            .instrument_await(format!("opendal::{}", Operation::Write.into_static()))
167    }
168
169    fn abort(&mut self) -> impl Future<Output = Result<()>> + MaybeSend {
170        self.inner
171            .abort()
172            .instrument_await(format!("opendal::{}", Operation::Write.into_static()))
173    }
174
175    fn close(&mut self) -> impl Future<Output = Result<Metadata>> + MaybeSend {
176        self.inner
177            .close()
178            .instrument_await(format!("opendal::{}", Operation::Write.into_static()))
179    }
180}
181
182impl<R: oio::List> oio::List for AwaitTreeWrapper<R> {
183    async fn next(&mut self) -> Result<Option<oio::Entry>> {
184        self.inner
185            .next()
186            .instrument_await(format!("opendal::{}", Operation::List))
187            .await
188    }
189}
190
191impl<R: oio::Delete> oio::Delete for AwaitTreeWrapper<R> {
192    fn delete(&mut self, path: &str, args: OpDelete) -> Result<()> {
193        self.inner.delete(path, args)
194    }
195
196    async fn flush(&mut self) -> Result<usize> {
197        self.inner
198            .flush()
199            .instrument_await(format!("opendal::{}", Operation::Delete))
200            .await
201    }
202}
```
