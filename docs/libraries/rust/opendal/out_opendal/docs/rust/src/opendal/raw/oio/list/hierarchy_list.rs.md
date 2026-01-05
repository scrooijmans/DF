# 

opendal/raw/oio/list/

hierarchy_list.rs

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
18use std::collections::HashSet;
19
20use crate::raw::*;
21use crate::*;
22
23/// ToHierarchyLister will convert a flat list to hierarchy by filter
24/// not needed entries.
25///
26/// # Notes
27///
28/// ToHierarchyLister filter entries after fetch entries. So it's possible
29/// to return an empty vec. It doesn't mean the all pages have been
30/// returned.
31///
32/// Please keep calling next until we returned `Ok(None)`
33pub struct HierarchyLister<P> {
34    lister: P,
35    path: String,
36    visited: HashSet<String>,
37    recursive: bool,
38}
39
40impl<P> HierarchyLister<P> {
41    /// Create a new hierarchy lister
42    pub fn new(lister: P, path: &str, recursive: bool) -> HierarchyLister<P> {
43        let path = if path == "/" {
44            "".to_string()
45        } else {
46            path.to_string()
47        };
48
49        HierarchyLister {
50            lister,
51            path,
52            visited: HashSet::default(),
53            recursive,
54        }
55    }
56
57    /// ## NOTES
58    ///
59    /// We take `&mut Entry` here because we need to perform modification on entry in the case like
60    /// listing "a/" with existing key `a/b/c`.
61    ///
62    /// In this case, we got a key `a/b/c`, but we should return `a/b/` instead to keep the hierarchy.
63    fn keep_entry(&mut self, e: &mut oio::Entry) -> bool {
64        // If path is not started with prefix, drop it.
65        //
66        // Ideally, it should never happen. But we just tolerate
67        // this state.
68        if !e.path().starts_with(&self.path) {
69            return false;
70        }
71
72        // Don't return already visited path.
73        if self.visited.contains(e.path()) {
74            return false;
75        }
76
77        let prefix_len = self.path.len();
78
79        let idx = if let Some(idx) = e.path()[prefix_len..].find('/') {
80            idx + prefix_len + 1
81        } else {
82            // If there is no `/` in path, it's a normal file, we
83            // can return it directly.
84            return true;
85        };
86
87        // idx == path.len() means it's contain only one `/` at the
88        // end of path.
89        if idx == e.path().len() {
90            if !self.visited.contains(e.path()) {
91                self.visited.insert(e.path().to_string());
92            }
93            return true;
94        }
95
96        // If idx < path.len() means that are more levels to come.
97        // We should check the first dir path.
98        let has = {
99            let path = &e.path()[..idx];
100            self.visited.contains(path)
101        };
102        if !has {
103            let path = {
104                let path = &e.path()[..idx];
105                path.to_string()
106            };
107
108            e.set_path(&path);
109            e.set_mode(EntryMode::DIR);
110            self.visited.insert(path);
111
112            return true;
113        }
114
115        false
116    }
117}
118
119impl<P: oio::List> oio::List for HierarchyLister<P> {
120    async fn next(&mut self) -> Result<Option<oio::Entry>> {
121        loop {
122            let mut entry = match self.lister.next().await? {
123                Some(entry) => entry,
124                None => return Ok(None),
125            };
126
127            if self.recursive {
128                return Ok(Some(entry));
129            }
130            if self.keep_entry(&mut entry) {
131                return Ok(Some(entry));
132            }
133        }
134    }
135}
```
