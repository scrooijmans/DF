# 

opendal/services/persy/

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
18use serde::Deserialize;
19use serde::Serialize;
20
21use super::backend::PersyBuilder;
22
23/// Config for persy service support.
24#[derive(Default, Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
25#[serde(default)]
26#[non_exhaustive]
27pub struct PersyConfig {
28    /// That path to the persy data file. The directory in the path must already exist.
29    pub datafile: Option<String>,
30    /// That name of the persy segment.
31    pub segment: Option<String>,
32    /// That name of the persy index.
33    pub index: Option<String>,
34}
35
36impl crate::Configurator for PersyConfig {
37    type Builder = PersyBuilder;
38
39    fn from_uri(uri: &crate::types::OperatorUri) -> crate::Result<Self> {
40        let mut map = uri.options().clone();
41
42        if let Some(path) = uri.root() {
43            if !path.is_empty() {
44                map.entry("datafile".to_string())
45                    .or_insert_with(|| format!("/{path}"));
46            }
47        }
48
49        Self::from_iter(map)
50    }
51
52    fn into_builder(self) -> Self::Builder {
53        PersyBuilder { config: self }
54    }
55}
56
57#[cfg(test)]
58mod tests {
59    use super::*;
60    use crate::Configurator;
61    use crate::types::OperatorUri;
62
63    #[test]
64    fn from_uri_sets_datafile_segment_and_index() {
65        let uri = OperatorUri::new(
66            "persy:///var/data/persy?segment=segment&index=index",
67            Vec::<(String, String)>::new(),
68        )
69        .unwrap();
70
71        let cfg = PersyConfig::from_uri(&uri).unwrap();
72        assert_eq!(cfg.datafile.as_deref(), Some("/var/data/persy"));
73        assert_eq!(cfg.segment.as_deref(), Some("segment"));
74        assert_eq!(cfg.index.as_deref(), Some("index"));
75    }
76}
```
