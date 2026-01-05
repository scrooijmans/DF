# 

opendal/

lib.rs

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
18#![doc(
19    html_logo_url = "https://raw.githubusercontent.com/apache/opendal/main/website/static/img/logo.svg"
20)]
21#![cfg_attr(docsrs, feature(doc_cfg))]
22//! Apache OpenDALâ¢ is an Open Data Access Layer that enables seamless interaction with diverse storage services.
23//!
24//! OpenDAL's development is guided by its vision of **One Layer, All Storage** and its core principles: **Open Community**, **Solid Foundation**, **Fast Access**, **Object Storage First**, and **Extensible Architecture**. Read the explained vision at [OpenDAL Vision](https://opendal.apache.org/vision).
25//!
26//! # Quick Start
27//!
28//! OpenDAL's API entry points are [`Operator`] and [`blocking::Operator`]. All
29//! public APIs are accessible through the operator. To utilize OpenDAL, you
30//! need to:
31//!
32//! - [Init a service](#init-a-service)
33//! - [Compose layers](#compose-layers)
34//! - [Use operator](#use-operator)
35//!
36//! ## Init a service
37//!
38//! The first step is to pick a service and init it with a builder. All supported
39//! services could be found at [`services`].
40//!
41//! Let's take [`services::S3`] as an example:
42//!
43//! ```no_run
44//! use opendal::services;
45//! use opendal::Operator;
46//! use opendal::Result;
47//!
48//! fn main() -> Result<()> {
49//!     // Pick a builder and configure it.
50//!     let mut builder = services::S3::default().bucket("test");
51//!
52//!     // Init an operator
53//!     let op = Operator::new(builder)?.finish();
54//!     Ok(())
55//! }
56//! ```
57//!
58//! ## Compose layers
59//!
60//! The next setup is to compose layers. Layers are modules that provide extra
61//! features for every operation. All builtin layers could be found at [`layers`].
62//!
63//! Let's use [`layers::LoggingLayer`] as an example; this layer adds logging to
64//! every operation that OpenDAL performs.
65//!
66//! ```no_run
67//! use opendal::layers::LoggingLayer;
68//! use opendal::services;
69//! use opendal::Operator;
70//! use opendal::Result;
71//!
72//! #[tokio::main]
73//! async fn main() -> Result<()> {
74//!     // Pick a builder and configure it.
75//!     let mut builder = services::S3::default().bucket("test");
76//!
77//!     // Init an operator
78//!     let op = Operator::new(builder)?
79//!         // Init with logging layer enabled.
80//!         .layer(LoggingLayer::default())
81//!         .finish();
82//!
83//!     Ok(())
84//! }
85//! ```
86//!
87//! ## Use operator
88//!
89//! The final step is to use the operator. OpenDAL supports both async [`Operator`]
90//! and blocking [`blocking::Operator`]. Please pick the one that fits your use case.
91//!
92//! Every Operator API follows a consistent pattern. For example, consider the `read` operation:
93//!
94//! - [`Operator::read`]: Executes a read operation.
95//! - [`Operator::read_with`]: Executes a read operation with additional options using the builder pattern.
96//! - [`Operator::read_options`]: Executes a read operation with extra options provided via a [`options::ReadOptions`] struct.
97//! - [`Operator::reader`]: Creates a reader for streaming data, allowing for flexible access.
98//! - [`Operator::reader_with`]: Creates a reader with advanced options using the builder pattern.
99//! - [`Operator::reader_options`]: Creates a reader with extra options provided via a [`options::ReadOptions`] struct.
100//!
101//! The [`Reader`] created by [`Operator`] supports custom read control methods and can be converted
102//! into [`futures::AsyncRead`] or [`futures::Stream`] for broader ecosystem compatibility.
103//!
104//! ```no_run
105//! use opendal::layers::LoggingLayer;
106//! use opendal::options;
107//! use opendal::services;
108//! use opendal::Operator;
109//! use opendal::Result;
110//!
111//! #[tokio::main]
112//! async fn main() -> Result<()> {
113//!     // Pick a builder and configure it.
114//!     let mut builder = services::S3::default().bucket("test");
115//!
116//!     // Init an operator
117//!     let op = Operator::new(builder)?
118//!         // Init with logging layer enabled.
119//!         .layer(LoggingLayer::default())
120//!         .finish();
121//!
122//!     // Fetch this file's metadata
123//!     let meta = op.stat("hello.txt").await?;
124//!     let length = meta.content_length();
125//!
126//!     // Read data from `hello.txt` with options.
127//!     let bs = op
128//!         .read_with("hello.txt")
129//!         .range(0..8 * 1024 * 1024)
130//!         .chunk(1024 * 1024)
131//!         .concurrent(4)
132//!         .await?;
133//!
134//!     // The same to:
135//!     let bs = op
136//!         .read_options("hello.txt", options::ReadOptions {
137//!             range: (0..8 * 1024 * 1024).into(),
138//!             chunk: Some(1024 * 1024),
139//!             concurrent: 4,
140//!             ..Default::default()
141//!         })
142//!         .await?;
143//!
144//!     Ok(())
145//! }
146//! ```
147//!
148//! # Useful Links
149//!
150//! - [Concept][crate::docs::concepts]
151//! - [Internals][crate::docs::internals]
152//! - [Performance Guide][crate::docs::performance]
153
154// Make sure all our public APIs have docs.
155#![deny(missing_docs)]
156
157// Private module with public types, they will be accessed via `opendal::Xxxx`
158mod types;
159pub use types::*;
160
161// Public modules, they will be accessed like `opendal::layers::Xxxx`
162#[cfg(feature = "blocking")]
163pub mod blocking;
164#[cfg(docsrs)]
165pub mod docs;
166pub mod layers;
167pub mod raw;
168pub mod services;
169
170#[cfg(test)]
171mod tests {
172    use std::mem::size_of;
173
174    use super::*;
175    /// This is not a real test case.
176    ///
177    /// We assert our public structs here to make sure we don't introduce
178    /// unexpected struct/enum size change.
179    #[cfg(target_pointer_width = "64")]
180    #[test]
181    fn assert_size() {
182        assert_eq!(16, size_of::<Operator>());
183        assert_eq!(360, size_of::<Entry>());
184        assert_eq!(336, size_of::<Metadata>());
185        assert_eq!(1, size_of::<EntryMode>());
186        assert_eq!(24, size_of::<Scheme>());
187    }
188
189    trait AssertSendSync: Send + Sync {}
190    impl AssertSendSync for Entry {}
191    impl AssertSendSync for Capability {}
192    impl AssertSendSync for Error {}
193    impl AssertSendSync for Reader {}
194    impl AssertSendSync for Writer {}
195    impl AssertSendSync for Lister {}
196    impl AssertSendSync for Operator {}
197
198    /// This is used to make sure our public API implement Send + Sync
199    #[test]
200    fn test_trait() {
201        let _: Box<dyn AssertSendSync> = Box::new(Capability::default());
202    }
203}
```
