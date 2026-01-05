# 

opendal/types/operator/

registry.rs

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
18use std::collections::HashMap;
19use std::sync::{LazyLock, Mutex};
20
21use crate::types::builder::{Builder, Configurator};
22use crate::types::{IntoOperatorUri, OperatorUri};
23use crate::{Error, ErrorKind, Operator, Result};
24
25/// Factory signature used to construct [`Operator`] from a URI and extra options.
26pub type OperatorFactory = fn(&OperatorUri) -> Result<Operator>;
27
28/// Default registry initialized with builtin services.
29pub static DEFAULT_OPERATOR_REGISTRY: LazyLock<OperatorRegistry> = LazyLock::new(|| {
30    let registry = OperatorRegistry::new();
31    register_builtin_services(&registry);
32    registry
33});
34
35/// Global registry that maps schemes to [`OperatorFactory`] functions.
36#[derive(Debug, Default)]
37pub struct OperatorRegistry {
38    factories: Mutex<HashMap<String, OperatorFactory>>,
39}
40
41impl OperatorRegistry {
42    /// Create a new, empty registry.
43    pub fn new() -> Self {
44        Self {
45            factories: Mutex::new(HashMap::new()),
46        }
47    }
48
49    /// Register a builder for the given scheme.
50    pub fn register<B: Builder>(&self, scheme: &str) {
51        let key = scheme.to_ascii_lowercase();
52        let mut guard = self
53            .factories
54            .lock()
55            .expect("operator registry mutex poisoned");
56        guard.insert(key, factory::<B::Config>);
57    }
58
59    /// Load an [`Operator`] via the factory registered for the URI's scheme.
60    pub fn load(&self, uri: impl IntoOperatorUri) -> Result<Operator> {
61        let parsed = uri.into_operator_uri()?;
62        let scheme = parsed.scheme();
63
64        let factory = self
65            .factories
66            .lock()
67            .expect("operator registry mutex poisoned")
68            .get(scheme)
69            .copied()
70            .ok_or_else(|| {
71                Error::new(ErrorKind::Unsupported, "scheme is not registered")
72                    .with_context("scheme", scheme.to_string())
73            })?;
74
75        factory(&parsed)
76    }
77}
78
79fn register_builtin_services(registry: &OperatorRegistry) {
80    let _ = registry;
81
82    #[cfg(feature = "services-memory")]
83    registry.register::<crate::services::Memory>(crate::services::MEMORY_SCHEME);
84    #[cfg(feature = "services-fs")]
85    registry.register::<crate::services::Fs>(crate::services::FS_SCHEME);
86    #[cfg(feature = "services-s3")]
87    registry.register::<crate::services::S3>(crate::services::S3_SCHEME);
88    #[cfg(feature = "services-azblob")]
89    registry.register::<crate::services::Azblob>(crate::services::AZBLOB_SCHEME);
90    #[cfg(feature = "services-b2")]
91    registry.register::<crate::services::B2>(crate::services::B2_SCHEME);
92    #[cfg(feature = "services-cos")]
93    registry.register::<crate::services::Cos>(crate::services::COS_SCHEME);
94    #[cfg(feature = "services-gcs")]
95    registry.register::<crate::services::Gcs>(crate::services::GCS_SCHEME);
96    #[cfg(feature = "services-obs")]
97    registry.register::<crate::services::Obs>(crate::services::OBS_SCHEME);
98    #[cfg(feature = "services-oss")]
99    registry.register::<crate::services::Oss>(crate::services::OSS_SCHEME);
100    #[cfg(feature = "services-upyun")]
101    registry.register::<crate::services::Upyun>(crate::services::UPYUN_SCHEME);
102}
103
104/// Factory adapter that builds an operator from a configurator type.
105fn factory<C: Configurator>(uri: &OperatorUri) -> Result<Operator> {
106    let cfg = C::from_uri(uri)?;
107    Ok(Operator::from_config(cfg)?.finish())
108}
```
