# 

opendal/raw/oio/list/

flat_list.rs

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
18use crate::raw::*;
19use crate::*;
20
21/// FlatLister will walk dir in bottom up way:
22///
23/// - List nested dir first
24/// - Go back into parent dirs one by one
25///
26/// Given the following file tree:
27///
28/// ```txt
29/// .
30/// âââ dir_x/
31/// â   âââ dir_y/
32/// â   â   âââ dir_z/
33/// â   â   âââ file_c
34/// â   âââ file_b
35/// âââ file_a
36/// ```
37///
38/// ToFlatLister will output entries like:
39///
40/// ```txt
41/// dir_x/dir_y/dir_z/file_c
42/// dir_x/dir_y/dir_z/
43/// dir_x/dir_y/file_b
44/// dir_x/dir_y/
45/// dir_x/file_a
46/// dir_x/
47/// ```
48///
49/// # Note
50///
51/// There is no guarantee about the order between files and dirs at the same level.
52/// We only make sure the nested dirs will show up before parent dirs.
53///
54/// Especially, for storage services that can't return dirs first, ToFlatLister
55/// may output parent dirs' files before nested dirs, this is expected because files
56/// always output directly while listing.
57pub struct FlatLister<A: Access, L> {
58    acc: A,
59
60    next_dir: Option<oio::Entry>,
61    active_lister: Vec<(Option<oio::Entry>, L)>,
62}
63
64/// # Safety
65///
66/// wasm32 is a special target that we only have one event-loop for this FlatLister.
67unsafe impl<A: Access, L> Send for FlatLister<A, L> {}
68/// # Safety
69///
70/// We will only take `&mut Self` reference for FsLister.
71unsafe impl<A: Access, L> Sync for FlatLister<A, L> {}
72
73impl<A, L> FlatLister<A, L>
74where
75    A: Access,
76{
77    /// Create a new flat lister
78    pub fn new(acc: A, path: &str) -> FlatLister<A, L> {
79        FlatLister {
80            acc,
81            next_dir: Some(oio::Entry::new(path, Metadata::new(EntryMode::DIR))),
82            active_lister: vec![],
83        }
84    }
85}
86
87impl<A, L> oio::List for FlatLister<A, L>
88where
89    A: Access<Lister = L>,
90    L: oio::List,
91{
92    async fn next(&mut self) -> Result<Option<oio::Entry>> {
93        loop {
94            if let Some(de) = self.next_dir.take() {
95                let (_, mut l) = self.acc.list(de.path(), OpList::new()).await?;
96                if let Some(v) = l.next().await? {
97                    self.active_lister.push((Some(de.clone()), l));
98
99                    if v.mode().is_dir() {
100                        // should not loop itself again
101                        if v.path() != de.path() {
102                            self.next_dir = Some(v);
103                            continue;
104                        }
105                    } else {
106                        return Ok(Some(v));
107                    }
108                }
109            }
110
111            let (de, lister) = match self.active_lister.last_mut() {
112                Some((de, lister)) => (de, lister),
113                None => return Ok(None),
114            };
115
116            match lister.next().await? {
117                Some(v) if v.mode().is_dir() => {
118                    // should not loop itself again
119                    if v.path() != de.as_ref().expect("de should not be none here").path() {
120                        self.next_dir = Some(v);
121                        continue;
122                    }
123                }
124                Some(v) => return Ok(Some(v)),
125                None => match de.take() {
126                    Some(de) => {
127                        return Ok(Some(de));
128                    }
129                    None => {
130                        let _ = self.active_lister.pop();
131                        continue;
132                    }
133                },
134            }
135        }
136    }
137}
```
