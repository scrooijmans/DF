# 

opendal/types/operator/

info.rs

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
18use std::str::FromStr;
19use std::sync::Arc;
20
21use crate::raw::*;
22use crate::*;
23
24/// Metadata for operator, users can use this metadata to get information of operator.
25#[derive(Clone, Debug, Default)]
26pub struct OperatorInfo(Arc<AccessorInfo>);
27
28impl OperatorInfo {
29    pub(super) fn new(acc: Arc<AccessorInfo>) -> Self {
30        OperatorInfo(acc)
31    }
32
33    /// [`Scheme`] of operator.
34    pub fn scheme(&self) -> Scheme {
35        let scheme_str = self.0.scheme();
36        Scheme::from_str(scheme_str).unwrap_or(Scheme::Custom(scheme_str))
37    }
38
39    /// Root of operator, will be in format like `/path/to/dir/`
40    pub fn root(&self) -> String {
41        self.0.root().to_string()
42    }
43
44    /// Name of backend, could be empty if underlying backend doesn't have namespace concept.
45    ///
46    /// For example:
47    ///
48    /// - name for `s3` => bucket name
49    /// - name for `azblob` => container name
50    pub fn name(&self) -> String {
51        self.0.name().to_string()
52    }
53
54    /// Get [`Full Capability`] of operator.
55    pub fn full_capability(&self) -> Capability {
56        self.0.full_capability()
57    }
58
59    /// Get [`Native Capability`] of operator.
60    pub fn native_capability(&self) -> Capability {
61        self.0.native_capability()
62    }
63}
```
