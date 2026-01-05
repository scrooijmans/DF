# 

opendal/docs/

concepts.rs

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
18//! The core concepts of OpenDAL's public API.
19//!
20//! OpenDAL provides a unified abstraction that helps developers access all storage services.
21//!
22//! There are two core concepts in OpenDAL:
23//!
24//! - [`Builder`]: Builder accepts a series of parameters to set up an instance of underlying services.
25//!   You can adjust the behaviour of underlying services with these parameters.
26//! - [`Operator`]: Developer can access underlying storage services with manipulating one Operator.
27//!   The Operator is a delegate for underlying implementation detail, and provides one unified access interface,
28//!   including `read`, `write`, `list` and so on.
29//!
30//! If you are interested in internal implementation details, please have a look at [`internals`][super::internals].
31//!
32//! # Builder
33//!
34//! Let's start with [`Builder`].
35//!
36//! A `Builder` is a trait that is implemented by the underlying services. We can use a `Builder` to configure and create a service.
37//! Developer can only create one service via Builder, in other words, Builder is the only public API provided by services.
38//! And other detailed implementation will be hidden.
39//!
40//! ```text
41//! âââââââââââââ                 âââââââââââââ
42//! â           â     build()     â           â
43//! â  Builder  ââââââââââââââââââºâ  Service  â
44//! â           â                 â           â
45//! âââââââââââââ                 âââââââââââââ
46//! ```
47//!
48//! All [`Builder`] provided by OpenDAL is under [`services`][crate::services], we can refer to them like `opendal::services::S3`.
49//! By right the builder will be named like `OneServiceBuilder`, but usually we will export it to public with renaming it as one
50//! general name. For example, we will rename `S3Builder` to `S3` and developer will use `S3` finally.
51//!
52//! For example:
53//!
54//! ```no_run
55//! use opendal::services::S3;
56//!
57//! let mut builder = S3::default();
58//! builder.bucket("example");
59//! builder.root("/path/to/file");
60//! ```
61//!
62//! # Operator
63//! The [`Operator`] is a delegate for Service, the underlying implementation detail that implements [`Access`][crate::raw::Access],
64//! and it also provides one unified access interface.
65//! It will hold one reference of Service with its all generic types erased by OpenDAL,
66//! which is the reason why we say the Operator is the delegate of one Service.
67//!
68//! ```text
69//!                   ââââââââââââââââââââââ
70//!                   â      Operator      â
71//!                   â         âdelegate  â
72//! âââââââââââ build â         â¼          â rely on âââââââââââââââââââââââ
73//! â Builder âââââââââ¼âââºââââââââââââââ   âââââââââââ¤ business logic code â
74//! âââââââââââ       â   â  Service   â   â         âââââââââââââââââââââââ
75//!                   âââââ´âââââââââââââ´ââââ
76//! ```
77//!
78//! `Operator` can be built from `Builder`:
79//!
80//! ```no_run
81//! # use opendal::Result;
82//! use opendal::services::S3;
83//! use opendal::Operator;
84//!
85//! # fn test() -> Result<()> {
86//! let mut builder = S3::default();
87//! builder.bucket("example");
88//! builder.root("/path/to/file");
89//!
90//! let op = Operator::new(builder)?.finish();
91//! # Ok(())
92//! # }
93//! ```
94//!
95//! - `Operator` has it's internal `Arc`, so it's **cheap** to clone it.
96//! - `Operator` doesn't have generic parameters or lifetimes, so it's **easy** to use it everywhere.
97//! - `Operator` implements `Send` and `Sync`, so it's **safe** to send it between threads.
98//!
99//! After get an `Operator`, we can do operations on different paths.
100//!
101//!
102//! ```text
103//!                            ââââââââââââââââ
104//!                  ââââââââââºâ read("abc")  â
105//!                  â         ââââââââââââââââ
106//! âââââââââââââ    â
107//! â Operator  â    â         ââââââââââââââââ
108//! â âââââââââ ââââââ¼âââââââââºâ write("def") â
109//! â âServiceâ â    â         ââââââââââââââââ
110//! âââ´ââââââââ´ââ    â
111//!                  â         ââââââââââââââââ
112//!                  ââââââââââºâ list("ghi/") â
113//!                            ââââââââââââââââ
114//! ```
115//!
116//! We can read data with given path in this way:
117//!
118//! ```no_run
119//! # use opendal::Result;
120//! use opendal::services::S3;
121//! use opendal::Operator;
122//!
123//! # async fn test() -> Result<()> {
124//! let mut builder = S3::default();
125//! builder.bucket("example");
126//! builder.root("/path/to/file");
127//!
128//! let op = Operator::new(builder)?.finish();
129//! let bs: Vec<u8> = op.read("abc").await?;
130//! # Ok(())
131//! # }
132//! ```
133//!
134//! [`Builder`]: crate::Builder
135//! [`Operator`]: crate::Operator
```
