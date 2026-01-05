# 

opendal/raw/

path_cache.rs

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
18use std::collections::VecDeque;
19
20use futures::Future;
21use moka::sync::Cache;
22use tokio::sync::Mutex;
23use tokio::sync::MutexGuard;
24
25use crate::raw::*;
26use crate::*;
27
28/// The trait required for path cacher.
29pub trait PathQuery {
30    /// Fetch the id for the root of the service.
31    fn root(&self) -> impl Future<Output = Result<String>> + MaybeSend;
32    /// Query the id by parent_id and name.
33    fn query(
34        &self,
35        parent_id: &str,
36        name: &str,
37    ) -> impl Future<Output = Result<Option<String>>> + MaybeSend;
38    /// Create a dir by parent_id and name.
39    fn create_dir(
40        &self,
41        parent_id: &str,
42        name: &str,
43    ) -> impl Future<Output = Result<String>> + MaybeSend;
44}
45
46/// PathCacher is a cache for path query.
47///
48/// OpenDAL is designed for path based storage systems, such as S3, HDFS, etc. But there are many
49/// services that are not path based, such as OneDrive, Google Drive, etc. For these services, we
50/// look up files based on id. The lookup of id is very expensive, so we cache the path to id mapping
51/// in PathCacher.
52///
53/// # Behavior
54///
55/// The `path` in the cache is always an absolute one. For example, if the service root is `/root/`,
56/// then the path of file `a/b` in cache will be `/root/a/b`.
57pub struct PathCacher<Q: PathQuery> {
58    query: Q,
59    cache: Cache<String, String>,
60
61    /// This optional lock here is used to prevent concurrent insertions of the same path.
62    ///
63    /// Some services like gdrive allows the same name to exist in the same directory. We need to introduce
64    /// a global lock to prevent concurrent insertions of the same path.
65    lock: Option<Mutex<()>>,
66}
67
68impl<Q: PathQuery> PathCacher<Q> {
69    /// Create a new path cacher.
70    pub fn new(query: Q) -> Self {
71        Self {
72            query,
73            cache: Cache::new(64 * 1024),
74            lock: None,
75        }
76    }
77
78    /// Enable the lock for the path cacher.
79    pub fn with_lock(mut self) -> Self {
80        self.lock = Some(Mutex::default());
81        self
82    }
83
84    async fn lock(&self) -> Option<MutexGuard<'_, ()>> {
85        if let Some(l) = &self.lock {
86            Some(l.lock().await)
87        } else {
88            None
89        }
90    }
91
92    /// Insert a new cache entry.
93    pub async fn insert(&self, path: &str, id: &str) {
94        let _guard = self.lock().await;
95
96        // This should never happen, but let's ignore the insert if happened.
97        if self.cache.contains_key(path) {
98            debug_assert!(
99                self.cache.get(path) == Some(id.to_string()),
100                "path {path} exists but it's value is inconsistent"
101            );
102            return;
103        }
104
105        self.cache.insert(path.to_string(), id.to_string());
106    }
107
108    /// Remove a cache entry.
109    pub async fn remove(&self, path: &str) {
110        let _guard = self.lock().await;
111
112        self.cache.invalidate(path)
113    }
114
115    /// Get the id for the given path.
116    pub async fn get(&self, path: &str) -> Result<Option<String>> {
117        let _guard = self.lock().await;
118
119        if let Some(id) = self.cache.get(path) {
120            return Ok(Some(id));
121        }
122
123        let mut paths = VecDeque::new();
124        let mut current_path = path;
125
126        while current_path != "/" && !current_path.is_empty() {
127            paths.push_front(current_path.to_string());
128            current_path = get_parent(current_path);
129            if let Some(id) = self.cache.get(current_path) {
130                return self.query_down(&id, paths).await;
131            }
132        }
133
134        let root_id = self.query.root().await?;
135        self.cache.insert("/".to_string(), root_id.clone());
136        self.query_down(&root_id, paths).await
137    }
138
139    /// `start_id` is the `file_id` to the start dir to query down.
140    /// `paths` is in the order like `["/a/", "/a/b/", "/a/b/c/"]`.
141    ///
142    /// We should fetch the next `file_id` by sending `query`.
143    async fn query_down(&self, start_id: &str, paths: VecDeque<String>) -> Result<Option<String>> {
144        let mut current_id = start_id.to_string();
145        for path in paths.into_iter() {
146            let name = get_basename(&path);
147            current_id = match self.query.query(&current_id, name).await? {
148                Some(id) => {
149                    self.cache.insert(path, id.clone());
150                    id
151                }
152                None => return Ok(None),
153            };
154        }
155        Ok(Some(current_id))
156    }
157
158    /// Ensure input dir exists.
159    pub async fn ensure_dir(&self, path: &str) -> Result<String> {
160        let _guard = self.lock().await;
161
162        let mut tmp = "".to_string();
163        // All parents that need to check.
164        let mut parents = vec![];
165        for component in path.split('/') {
166            if component.is_empty() {
167                continue;
168            }
169
170            tmp.push_str(component);
171            tmp.push('/');
172            parents.push(tmp.to_string());
173        }
174
175        let mut parent_id = match self.cache.get("/") {
176            Some(v) => v,
177            None => self.query.root().await?,
178        };
179        for parent in parents {
180            parent_id = match self.cache.get(&parent) {
181                Some(value) => value,
182                None => {
183                    let value = match self.query.query(&parent_id, get_basename(&parent)).await? {
184                        Some(value) => value,
185                        None => {
186                            self.query
187                                .create_dir(&parent_id, get_basename(&parent))
188                                .await?
189                        }
190                    };
191                    self.cache.insert(parent, value.clone());
192                    value
193                }
194            }
195        }
196
197        Ok(parent_id)
198    }
199}
200
201#[cfg(test)]
202mod tests {
203
204    use crate::raw::PathCacher;
205    use crate::raw::PathQuery;
206    use crate::*;
207
208    struct TestQuery {}
209
210    impl PathQuery for TestQuery {
211        async fn root(&self) -> Result<String> {
212            Ok("root/".to_string())
213        }
214
215        async fn query(&self, parent_id: &str, name: &str) -> Result<Option<String>> {
216            if name.starts_with("not_exist") {
217                return Ok(None);
218            }
219            Ok(Some(format!("{parent_id}{name}")))
220        }
221
222        async fn create_dir(&self, parent_id: &str, name: &str) -> Result<String> {
223            Ok(format!("{parent_id}{name}"))
224        }
225    }
226
227    #[tokio::test]
228    async fn test_path_cacher_get() {
229        let cases = vec![
230            ("root", "/", Some("root/")),
231            ("normal path", "/a", Some("root/a")),
232            ("not exist normal dir", "/not_exist/a", None),
233            ("not exist normal file", "/a/b/not_exist", None),
234            ("nest path", "/a/b/c/d", Some("root/a/b/c/d")),
235        ];
236
237        for (name, input, expect) in cases {
238            let cache = PathCacher::new(TestQuery {});
239
240            let actual = cache.get(input).await.unwrap();
241            assert_eq!(actual.as_deref(), expect, "{name}")
242        }
243    }
244}
```
