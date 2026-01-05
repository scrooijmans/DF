# 

opendal/services/d1/

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
21use serde::Deserialize;
22use serde::Serialize;
23
24use super::backend::D1Builder;
25
26/// Config for [Cloudflare D1](https://developers.cloudflare.com/d1) backend support.
27#[derive(Default, Serialize, Deserialize, Clone, PartialEq, Eq)]
28#[serde(default)]
29#[non_exhaustive]
30pub struct D1Config {
31    /// Set the token of cloudflare api.
32    pub token: Option<String>,
33    /// Set the account id of cloudflare api.
34    pub account_id: Option<String>,
35    /// Set the database id of cloudflare api.
36    pub database_id: Option<String>,
37
38    /// Set the working directory of OpenDAL.
39    pub root: Option<String>,
40    /// Set the table of D1 Database.
41    pub table: Option<String>,
42    /// Set the key field of D1 Database.
43    pub key_field: Option<String>,
44    /// Set the value field of D1 Database.
45    pub value_field: Option<String>,
46}
47
48impl Debug for D1Config {
49    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
50        let mut ds = f.debug_struct("D1Config");
51        ds.field("root", &self.root);
52        ds.field("table", &self.table);
53        ds.field("key_field", &self.key_field);
54        ds.field("value_field", &self.value_field);
55        ds.finish_non_exhaustive()
56    }
57}
58
59impl crate::Configurator for D1Config {
60    type Builder = D1Builder;
61
62    fn from_uri(uri: &crate::types::OperatorUri) -> crate::Result<Self> {
63        let account_id = uri.name().ok_or_else(|| {
64            crate::Error::new(
65                crate::ErrorKind::ConfigInvalid,
66                "uri host must contain account id",
67            )
68            .with_context("service", crate::Scheme::D1)
69        })?;
70
71        let database_and_root = uri.root().ok_or_else(|| {
72            crate::Error::new(
73                crate::ErrorKind::ConfigInvalid,
74                "uri path must contain database id",
75            )
76            .with_context("service", crate::Scheme::D1)
77        })?;
78
79        let mut segments = database_and_root.splitn(2, '/');
80        let database_id = segments.next().filter(|s| !s.is_empty()).ok_or_else(|| {
81            crate::Error::new(
82                crate::ErrorKind::ConfigInvalid,
83                "database id is required in uri path",
84            )
85            .with_context("service", crate::Scheme::D1)
86        })?;
87
88        let mut map = uri.options().clone();
89        map.insert("account_id".to_string(), account_id.to_string());
90        map.insert("database_id".to_string(), database_id.to_string());
91
92        if let Some(rest) = segments.next() {
93            if !rest.is_empty() {
94                map.insert("root".to_string(), rest.to_string());
95            }
96        }
97
98        Self::from_iter(map)
99    }
100
101    fn into_builder(self) -> Self::Builder {
102        D1Builder {
103            config: self,
104            http_client: None,
105        }
106    }
107}
108
109#[cfg(test)]
110mod tests {
111    use super::*;
112    use crate::Configurator;
113    use crate::types::OperatorUri;
114
115    #[test]
116    fn from_uri_sets_account_database_and_root() {
117        let uri =
118            OperatorUri::new("d1://acc123/db456/cache", Vec::<(String, String)>::new()).unwrap();
119
120        let cfg = D1Config::from_uri(&uri).unwrap();
121        assert_eq!(cfg.account_id.as_deref(), Some("acc123"));
122        assert_eq!(cfg.database_id.as_deref(), Some("db456"));
123        assert_eq!(cfg.root.as_deref(), Some("cache"));
124    }
125}
```
