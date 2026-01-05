# 

opendal/types/

builder.rs

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
18use std::fmt::Debug;
19
20use serde::Serialize;
21use serde::de::DeserializeOwned;
22
23use crate::raw::*;
24use crate::types::OperatorUri;
25use crate::*;
26
27/// Builder is used to set up underlying services.
28///
29/// This trait allows the developer to define a builder struct that can:
30///
31/// - build a service via builder style API.
32/// - configure in-memory options like `http_client` or `customized_credential_load`.
33///
34/// Usually, users don't need to use or import this trait directly, they can use `Operator` API instead.
35///
36/// For example:
37///
38/// ```
39/// # use anyhow::Result;
40/// use opendal::services::Fs;
41/// use opendal::Operator;
42/// async fn test() -> Result<()> {
43///     // Create fs backend builder.
44///     let mut builder = Fs::default().root("/tmp");
45///
46///     // Build an `Operator` to start operating the storage.
47///     let op: Operator = Operator::new(builder)?.finish();
48///
49///     Ok(())
50/// }
51/// ```
52pub trait Builder: Default + 'static {
53    /// Associated configuration for this builder.
54    type Config: Configurator;
55
56    /// Consume the accessor builder to build a service.
57    fn build(self) -> Result<impl Access>;
58}
59
60/// Dummy implementation of builder
61impl Builder for () {
62    type Config = ();
63
64    fn build(self) -> Result<impl Access> {
65        Ok(())
66    }
67}
68
69/// Configurator is used to configure the underlying service.
70///
71/// This trait allows the developer to define a configuration struct that can:
72///
73/// - deserialize from an iterator like hashmap or vector.
74/// - convert into a service builder and finally build the underlying services.
75///
76/// Usually, users don't need to use or import this trait directly, they can use `Operator` API instead.
77///
78/// For example:
79///
80/// ```
81/// # use anyhow::Result;
82/// use std::collections::HashMap;
83///
84/// use opendal::services::MemoryConfig;
85/// use opendal::Operator;
86/// async fn test() -> Result<()> {
87///     let mut cfg = MemoryConfig::default();
88///     cfg.root = Some("/".to_string());
89///
90///     // Build an `Operator` to start operating the storage.
91///     let op: Operator = Operator::from_config(cfg)?.finish();
92///
93///     Ok(())
94/// }
95/// ```
96///
97/// Some service builder might contain in memory options like `http_client` . Users can call
98/// `into_builder` to convert the configuration into a builder instead.
99///
100/// ```
101/// # use anyhow::Result;
102/// use std::collections::HashMap;
103///
104/// use opendal::raw::HttpClient;
105/// use opendal::services::S3Config;
106/// use opendal::Configurator;
107/// use opendal::Operator;
108///
109/// async fn test() -> Result<()> {
110///     let mut cfg = S3Config::default();
111///     cfg.root = Some("/".to_string());
112///     cfg.bucket = "test".to_string();
113///
114///     let builder = cfg.into_builder();
115///     let builder = builder.http_client(HttpClient::new()?);
116///
117///     // Build an `Operator` to start operating the storage.
118///     let op: Operator = Operator::new(builder)?.finish();
119///
120///     Ok(())
121/// }
122/// ```
123pub trait Configurator: Serialize + DeserializeOwned + Debug + 'static {
124    /// Associated builder for this configuration.
125    type Builder: Builder;
126
127    /// Build configuration from a parsed URI plus merged options.
128    fn from_uri(_uri: &OperatorUri) -> Result<Self> {
129        Err(Error::new(ErrorKind::Unsupported, "uri is not supported"))
130    }
131
132    /// Deserialize from an iterator.
133    ///
134    /// This API is provided by opendal, developer should not implement it.
135    fn from_iter(iter: impl IntoIterator<Item = (String, String)>) -> Result<Self> {
136        let cfg = ConfigDeserializer::new(iter.into_iter().collect());
137
138        Self::deserialize(cfg).map_err(|err| {
139            Error::new(ErrorKind::ConfigInvalid, "failed to deserialize config").set_source(err)
140        })
141    }
142
143    /// Convert this configuration into a service builder.
144    fn into_builder(self) -> Self::Builder;
145}
146
147impl Configurator for () {
148    type Builder = ();
149
150    fn into_builder(self) -> Self::Builder {}
151}
```
