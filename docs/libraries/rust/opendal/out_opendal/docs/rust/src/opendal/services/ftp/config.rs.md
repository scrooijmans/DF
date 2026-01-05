# 

opendal/services/ftp/

config.rs

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
19use std::fmt::Formatter;
20
21use super::FTP_SCHEME;
22use super::backend::FtpBuilder;
23use serde::Deserialize;
24use serde::Serialize;
25
26/// Config for Ftp services support.
27#[derive(Default, Serialize, Deserialize, Clone, PartialEq, Eq)]
28#[serde(default)]
29#[non_exhaustive]
30pub struct FtpConfig {
31    /// endpoint of this backend
32    pub endpoint: Option<String>,
33    /// root of this backend
34    pub root: Option<String>,
35    /// user of this backend
36    pub user: Option<String>,
37    /// password of this backend
38    pub password: Option<String>,
39}
40
41impl Debug for FtpConfig {
42    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
43        f.debug_struct("FtpConfig")
44            .field("endpoint", &self.endpoint)
45            .field("root", &self.root)
46            .finish_non_exhaustive()
47    }
48}
49
50impl crate::Configurator for FtpConfig {
51    type Builder = FtpBuilder;
52
53    fn from_uri(uri: &crate::types::OperatorUri) -> crate::Result<Self> {
54        let authority = uri.authority().ok_or_else(|| {
55            crate::Error::new(crate::ErrorKind::ConfigInvalid, "uri authority is required")
56                .with_context("service", crate::Scheme::Ftp)
57        })?;
58
59        let mut map = uri.options().clone();
60        map.insert(
61            "endpoint".to_string(),
62            format!("{FTP_SCHEME}://{authority}"),
63        );
64
65        if let Some(root) = uri.root() {
66            map.insert("root".to_string(), root.to_string());
67        }
68
69        Self::from_iter(map)
70    }
71
72    fn into_builder(self) -> Self::Builder {
73        FtpBuilder { config: self }
74    }
75}
```
