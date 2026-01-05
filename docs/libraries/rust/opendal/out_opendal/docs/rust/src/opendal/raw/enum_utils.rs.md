# 

opendal/raw/

enum_utils.rs

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
18//! [`type_alias_impl_trait`](https://github.com/rust-lang/rust/issues/63063) is not stable yet.
19//! So we can't write the following code:
20//!
21//! ```txt
22//! impl Access for S3Backend {
23//!     type Writer = impl oio::Write;
24//! }
25//! ```
26//!
27//! Which means we have to write the type directly like:
28//!
29//! ```txt
30//! impl Access for OssBackend {
31//!     type Writer = raw::TwoWays<
32//!         oio::MultipartWriter<OssWriter>,
33//!         oio::AppendWriter<OssWriter>,
34//!     >;
35//! }
36//! ```
37//!
38//! This module is used to provide some enums for the above code. We should remove this module once
39//! type_alias_impl_trait has been stabilized.
40
41use crate::raw::*;
42use crate::*;
43
44/// TwoWays is used to implement traits that based on two ways.
45///
46/// Users can wrap two different trait types together.
47pub enum TwoWays<ONE, TWO> {
48    /// The first type for the [`TwoWays`].
49    One(ONE),
50    /// The second type for the [`TwoWays`].
51    Two(TWO),
52}
53
54impl<ONE: oio::Read, TWO: oio::Read> oio::Read for TwoWays<ONE, TWO> {
55    async fn read(&mut self) -> Result<Buffer> {
56        match self {
57            TwoWays::One(v) => v.read().await,
58            TwoWays::Two(v) => v.read().await,
59        }
60    }
61}
62
63impl<ONE: oio::Write, TWO: oio::Write> oio::Write for TwoWays<ONE, TWO> {
64    async fn write(&mut self, bs: Buffer) -> Result<()> {
65        match self {
66            Self::One(v) => v.write(bs).await,
67            Self::Two(v) => v.write(bs).await,
68        }
69    }
70
71    async fn close(&mut self) -> Result<Metadata> {
72        match self {
73            Self::One(v) => v.close().await,
74            Self::Two(v) => v.close().await,
75        }
76    }
77
78    async fn abort(&mut self) -> Result<()> {
79        match self {
80            Self::One(v) => v.abort().await,
81            Self::Two(v) => v.abort().await,
82        }
83    }
84}
85
86impl<ONE: oio::List, TWO: oio::List> oio::List for TwoWays<ONE, TWO> {
87    async fn next(&mut self) -> Result<Option<oio::Entry>> {
88        match self {
89            Self::One(v) => v.next().await,
90            Self::Two(v) => v.next().await,
91        }
92    }
93}
94
95/// ThreeWays is used to implement traits that based on three ways.
96///
97/// Users can wrap three different trait types together.
98pub enum ThreeWays<ONE, TWO, THREE> {
99    /// The first type for the [`ThreeWays`].
100    One(ONE),
101    /// The second type for the [`ThreeWays`].
102    Two(TWO),
103    /// The third type for the [`ThreeWays`].
104    Three(THREE),
105}
106
107impl<ONE: oio::Read, TWO: oio::Read, THREE: oio::Read> oio::Read for ThreeWays<ONE, TWO, THREE> {
108    async fn read(&mut self) -> Result<Buffer> {
109        match self {
110            ThreeWays::One(v) => v.read().await,
111            ThreeWays::Two(v) => v.read().await,
112            ThreeWays::Three(v) => v.read().await,
113        }
114    }
115}
116
117impl<ONE: oio::Write, TWO: oio::Write, THREE: oio::Write> oio::Write
118    for ThreeWays<ONE, TWO, THREE>
119{
120    async fn write(&mut self, bs: Buffer) -> Result<()> {
121        match self {
122            Self::One(v) => v.write(bs).await,
123            Self::Two(v) => v.write(bs).await,
124            Self::Three(v) => v.write(bs).await,
125        }
126    }
127
128    async fn close(&mut self) -> Result<Metadata> {
129        match self {
130            Self::One(v) => v.close().await,
131            Self::Two(v) => v.close().await,
132            Self::Three(v) => v.close().await,
133        }
134    }
135
136    async fn abort(&mut self) -> Result<()> {
137        match self {
138            Self::One(v) => v.abort().await,
139            Self::Two(v) => v.abort().await,
140            Self::Three(v) => v.abort().await,
141        }
142    }
143}
144
145impl<ONE: oio::List, TWO: oio::List, THREE: oio::List> oio::List for ThreeWays<ONE, TWO, THREE> {
146    async fn next(&mut self) -> Result<Option<oio::Entry>> {
147        match self {
148            Self::One(v) => v.next().await,
149            Self::Two(v) => v.next().await,
150            Self::Three(v) => v.next().await,
151        }
152    }
153}
154
155/// FourWays is used to implement traits that based on four ways.
156///
157/// Users can wrap four different trait types together.
158pub enum FourWays<ONE, TWO, THREE, FOUR> {
159    /// The first type for the [`FourWays`].
160    One(ONE),
161    /// The second type for the [`FourWays`].
162    Two(TWO),
163    /// The third type for the [`FourWays`].
164    Three(THREE),
165    /// The fourth type for the [`FourWays`].
166    Four(FOUR),
167}
168
169impl<ONE, TWO, THREE, FOUR> oio::Read for FourWays<ONE, TWO, THREE, FOUR>
170where
171    ONE: oio::Read,
172    TWO: oio::Read,
173    THREE: oio::Read,
174    FOUR: oio::Read,
175{
176    async fn read(&mut self) -> Result<Buffer> {
177        match self {
178            FourWays::One(v) => v.read().await,
179            FourWays::Two(v) => v.read().await,
180            FourWays::Three(v) => v.read().await,
181            FourWays::Four(v) => v.read().await,
182        }
183    }
184}
185
186impl<ONE, TWO, THREE, FOUR> oio::List for FourWays<ONE, TWO, THREE, FOUR>
187where
188    ONE: oio::List,
189    TWO: oio::List,
190    THREE: oio::List,
191    FOUR: oio::List,
192{
193    async fn next(&mut self) -> Result<Option<oio::Entry>> {
194        match self {
195            Self::One(v) => v.next().await,
196            Self::Two(v) => v.next().await,
197            Self::Three(v) => v.next().await,
198            Self::Four(v) => v.next().await,
199        }
200    }
201}
```
