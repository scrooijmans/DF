# 

opendal/services/gridfs/

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
24use super::backend::GridfsBuilder;
25
26/// Config for Grid file system support.
27#[derive(Default, Serialize, Deserialize, Clone, PartialEq, Eq)]
28#[serde(default)]
29#[non_exhaustive]
30pub struct GridfsConfig {
31    /// The connection string of the MongoDB service.
32    pub connection_string: Option<String>,
33    /// The database name of the MongoDB GridFs service to read/write.
34    pub database: Option<String>,
35    /// The bucket name of the MongoDB GridFs service to read/write.
36    pub bucket: Option<String>,
37    /// The chunk size of the MongoDB GridFs service used to break the user file into chunks.
38    pub chunk_size: Option<u32>,
39    /// The working directory, all operations will be performed under it.
40    pub root: Option<String>,
41}
42
43impl Debug for GridfsConfig {
44    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
45        f.debug_struct("GridFsConfig")
46            .field("database", &self.database)
47            .field("bucket", &self.bucket)
48            .field("chunk_size", &self.chunk_size)
49            .field("root", &self.root)
50            .finish()
51    }
52}
53
54impl crate::Configurator for GridfsConfig {
55    type Builder = GridfsBuilder;
56
57    fn from_uri(uri: &crate::types::OperatorUri) -> crate::Result<Self> {
58        let mut map = uri.options().clone();
59
60        if let Some(authority) = uri.authority() {
61            map.entry("connection_string".to_string())
62                .or_insert_with(|| format!("mongodb://{authority}"));
63        }
64
65        if let Some(path) = uri.root() {
66            if !path.is_empty() {
67                let mut segments = path.splitn(3, '/');
68                if let Some(database) = segments.next() {
69                    if !database.is_empty() {
70                        map.entry("database".to_string())
71                            .or_insert_with(|| database.to_string());
72                    }
73                }
74                if let Some(bucket) = segments.next() {
75                    if !bucket.is_empty() {
76                        map.entry("bucket".to_string())
77                            .or_insert_with(|| bucket.to_string());
78                    }
79                }
80                if let Some(rest) = segments.next() {
81                    if !rest.is_empty() {
82                        map.insert("root".to_string(), rest.to_string());
83                    }
84                }
85            }
86        }
87
88        Self::from_iter(map)
89    }
90
91    fn into_builder(self) -> Self::Builder {
92        GridfsBuilder { config: self }
93    }
94}
95
96#[cfg(test)]
97mod tests {
98    use super::*;
99    use crate::Configurator;
100    use crate::types::OperatorUri;
101
102    #[test]
103    fn from_uri_sets_connection_database_bucket_and_root() {
104        let uri = OperatorUri::new(
105            "gridfs://mongo.example.com:27017/app_files/assets/images",
106            Vec::<(String, String)>::new(),
107        )
108        .unwrap();
109
110        let cfg = GridfsConfig::from_uri(&uri).unwrap();
111        assert_eq!(
112            cfg.connection_string.as_deref(),
113            Some("mongodb://mongo.example.com:27017")
114        );
115        assert_eq!(cfg.database.as_deref(), Some("app_files"));
116        assert_eq!(cfg.bucket.as_deref(), Some("assets"));
117        assert_eq!(cfg.root.as_deref(), Some("images"));
118    }
119}
```
