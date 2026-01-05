# 

opendal/layers/

immutable_index.rs

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
19use std::fmt::Debug;
20use std::vec::IntoIter;
21
22use crate::raw::*;
23use crate::*;
24
25/// Add an immutable in-memory index for underlying storage services.
26///
27/// Especially useful for services without list capability like HTTP.
28///
29/// # Examples
30///
31/// ```rust, no_run
32/// # use std::collections::HashMap;
33///
34/// # use opendal::layers::ImmutableIndexLayer;
35/// # use opendal::services;
36/// # use opendal::Operator;
37/// # use opendal::Result;
38///
39/// # fn main() -> Result<()> {
40/// let mut iil = ImmutableIndexLayer::default();
41///
42/// for i in ["file", "dir/", "dir/file", "dir_without_prefix/file"] {
43///     iil.insert(i.to_string())
44/// }
45///
46/// let op = Operator::from_iter::<services::Memory>(HashMap::<_, _>::default())?
47///     .layer(iil)
48///     .finish();
49/// Ok(())
50/// # }
51/// ```
52#[derive(Default, Debug, Clone)]
53pub struct ImmutableIndexLayer {
54    vec: Vec<String>,
55}
56
57impl ImmutableIndexLayer {
58    /// Insert a key into index.
59    pub fn insert(&mut self, key: String) {
60        self.vec.push(key);
61    }
62
63    /// Insert keys from iter.
64    pub fn extend_iter<I>(&mut self, iter: I)
65    where
66        I: IntoIterator<Item = String>,
67    {
68        self.vec.extend(iter);
69    }
70}
71
72impl<A: Access> Layer<A> for ImmutableIndexLayer {
73    type LayeredAccess = ImmutableIndexAccessor<A>;
74
75    fn layer(&self, inner: A) -> Self::LayeredAccess {
76        let info = inner.info();
77        info.update_full_capability(|mut cap| {
78            cap.list = true;
79            cap.list_with_recursive = true;
80            cap
81        });
82
83        ImmutableIndexAccessor {
84            vec: self.vec.clone(),
85            inner,
86        }
87    }
88}
89
90#[derive(Debug, Clone)]
91pub struct ImmutableIndexAccessor<A: Access> {
92    inner: A,
93    vec: Vec<String>,
94}
95
96impl<A: Access> ImmutableIndexAccessor<A> {
97    fn children_flat(&self, path: &str) -> Vec<String> {
98        self.vec
99            .iter()
100            .filter(|v| v.starts_with(path) && v.as_str() != path)
101            .cloned()
102            .collect()
103    }
104
105    fn children_hierarchy(&self, path: &str) -> Vec<String> {
106        let mut res = HashSet::new();
107
108        for i in self.vec.iter() {
109            // `/xyz` should not belong to `/abc`
110            if !i.starts_with(path) {
111                continue;
112            }
113
114            // remove `/abc` if self
115            if i == path {
116                continue;
117            }
118
119            match i[path.len()..].find('/') {
120                // File `/abc/def.csv` must belong to `/abc`
121                None => {
122                    res.insert(i.to_string());
123                }
124                Some(idx) => {
125                    // The index of first `/` after `/abc`.
126                    let dir_idx = idx + 1 + path.len();
127
128                    if dir_idx == i.len() {
129                        // Dir `/abc/def/` belongs to `/abc/`
130                        res.insert(i.to_string());
131                    } else {
132                        // File/Dir `/abc/def/xyz` doesn't belong to `/abc`.
133                        // But we need to list `/abc/def` out so that we can walk down.
134                        res.insert(i[..dir_idx].to_string());
135                    }
136                }
137            }
138        }
139
140        res.into_iter().collect()
141    }
142}
143
144impl<A: Access> LayeredAccess for ImmutableIndexAccessor<A> {
145    type Inner = A;
146    type Reader = A::Reader;
147    type Writer = A::Writer;
148    type Lister = ImmutableDir;
149    type Deleter = A::Deleter;
150
151    fn inner(&self) -> &Self::Inner {
152        &self.inner
153    }
154
155    async fn read(&self, path: &str, args: OpRead) -> Result<(RpRead, Self::Reader)> {
156        self.inner.read(path, args).await
157    }
158
159    async fn write(&self, path: &str, args: OpWrite) -> Result<(RpWrite, Self::Writer)> {
160        self.inner.write(path, args).await
161    }
162
163    async fn list(&self, path: &str, args: OpList) -> Result<(RpList, Self::Lister)> {
164        let mut path = path;
165        if path == "/" {
166            path = ""
167        }
168
169        let idx = if args.recursive() {
170            self.children_flat(path)
171        } else {
172            self.children_hierarchy(path)
173        };
174
175        Ok((RpList::default(), ImmutableDir::new(idx)))
176    }
177
178    async fn delete(&self) -> Result<(RpDelete, Self::Deleter)> {
179        self.inner.delete().await
180    }
181}
182
183pub struct ImmutableDir {
184    idx: IntoIter<String>,
185}
186
187impl ImmutableDir {
188    fn new(idx: Vec<String>) -> Self {
189        Self {
190            idx: idx.into_iter(),
191        }
192    }
193
194    fn inner_next(&mut self) -> Option<oio::Entry> {
195        self.idx.next().map(|v| {
196            let mode = if v.ends_with('/') {
197                EntryMode::DIR
198            } else {
199                EntryMode::FILE
200            };
201            let meta = Metadata::new(mode);
202            oio::Entry::with(v, meta)
203        })
204    }
205}
206
207impl oio::List for ImmutableDir {
208    async fn next(&mut self) -> Result<Option<oio::Entry>> {
209        Ok(self.inner_next())
210    }
211}
212
213#[cfg(test)]
214#[cfg(feature = "services-http")]
215mod tests {
216    use std::collections::HashMap;
217    use std::collections::HashSet;
218
219    use anyhow::Result;
220    use futures::TryStreamExt;
221    use log::debug;
222
223    use super::*;
224    use crate::EntryMode;
225    use crate::Operator;
226    use crate::layers::LoggingLayer;
227    use crate::services::HttpConfig;
228
229    #[tokio::test]
230    async fn test_list() -> Result<()> {
231        let _ = tracing_subscriber::fmt().with_test_writer().try_init();
232
233        let mut iil = ImmutableIndexLayer::default();
234        for i in ["file", "dir/", "dir/file", "dir_without_prefix/file"] {
235            iil.insert(i.to_string())
236        }
237
238        let op = HttpConfig::from_iter({
239            let mut map = HashMap::new();
240            map.insert("endpoint".to_string(), "https://xuanwo.io".to_string());
241            map
242        })
243        .and_then(Operator::from_config)?
244        .layer(LoggingLayer::default())
245        .layer(iil)
246        .finish();
247
248        let mut map = HashMap::new();
249        let mut set = HashSet::new();
250        let mut ds = op.lister("").await?;
251        while let Some(entry) = ds.try_next().await? {
252            debug!("got entry: {}", entry.path());
253            assert!(
254                set.insert(entry.path().to_string()),
255                "duplicated value: {}",
256                entry.path()
257            );
258            map.insert(entry.path().to_string(), entry.metadata().mode());
259        }
260
261        assert_eq!(map["file"], EntryMode::FILE);
262        assert_eq!(map["dir/"], EntryMode::DIR);
263        assert_eq!(map["dir_without_prefix/"], EntryMode::DIR);
264        Ok(())
265    }
266
267    #[tokio::test]
268    async fn test_scan() -> Result<()> {
269        let _ = tracing_subscriber::fmt().with_test_writer().try_init();
270
271        let mut iil = ImmutableIndexLayer::default();
272        for i in ["file", "dir/", "dir/file", "dir_without_prefix/file"] {
273            iil.insert(i.to_string())
274        }
275
276        let op = HttpConfig::from_iter({
277            let mut map = HashMap::new();
278            map.insert("endpoint".to_string(), "https://xuanwo.io".to_string());
279            map
280        })
281        .and_then(Operator::from_config)?
282        .layer(LoggingLayer::default())
283        .layer(iil)
284        .finish();
285
286        let mut ds = op.lister_with("/").recursive(true).await?;
287        let mut set = HashSet::new();
288        let mut map = HashMap::new();
289        while let Some(entry) = ds.try_next().await? {
290            debug!("got entry: {}", entry.path());
291            assert!(
292                set.insert(entry.path().to_string()),
293                "duplicated value: {}",
294                entry.path()
295            );
296            map.insert(entry.path().to_string(), entry.metadata().mode());
297        }
298
299        debug!("current files: {map:?}");
300
301        assert_eq!(map["file"], EntryMode::FILE);
302        assert_eq!(map["dir/"], EntryMode::DIR);
303        assert_eq!(map["dir_without_prefix/file"], EntryMode::FILE);
304        Ok(())
305    }
306
307    #[tokio::test]
308    async fn test_list_dir() -> Result<()> {
309        let _ = tracing_subscriber::fmt().with_test_writer().try_init();
310
311        let mut iil = ImmutableIndexLayer::default();
312        for i in [
313            "dataset/stateful/ontime_2007_200.csv",
314            "dataset/stateful/ontime_2008_200.csv",
315            "dataset/stateful/ontime_2009_200.csv",
316        ] {
317            iil.insert(i.to_string())
318        }
319
320        let op = HttpConfig::from_iter({
321            let mut map = HashMap::new();
322            map.insert("endpoint".to_string(), "https://xuanwo.io".to_string());
323            map
324        })
325        .and_then(Operator::from_config)?
326        .layer(LoggingLayer::default())
327        .layer(iil)
328        .finish();
329
330        //  List /
331        let mut map = HashMap::new();
332        let mut set = HashSet::new();
333        let mut ds = op.lister("/").await?;
334        while let Some(entry) = ds.try_next().await? {
335            assert!(
336                set.insert(entry.path().to_string()),
337                "duplicated value: {}",
338                entry.path()
339            );
340            map.insert(entry.path().to_string(), entry.metadata().mode());
341        }
342
343        assert_eq!(map.len(), 1);
344        assert_eq!(map["dataset/"], EntryMode::DIR);
345
346        //  List dataset/stateful/
347        let mut map = HashMap::new();
348        let mut set = HashSet::new();
349        let mut ds = op.lister("dataset/stateful/").await?;
350        while let Some(entry) = ds.try_next().await? {
351            assert!(
352                set.insert(entry.path().to_string()),
353                "duplicated value: {}",
354                entry.path()
355            );
356            map.insert(entry.path().to_string(), entry.metadata().mode());
357        }
358
359        assert_eq!(map["dataset/stateful/ontime_2007_200.csv"], EntryMode::FILE);
360        assert_eq!(map["dataset/stateful/ontime_2008_200.csv"], EntryMode::FILE);
361        assert_eq!(map["dataset/stateful/ontime_2009_200.csv"], EntryMode::FILE);
362        Ok(())
363    }
364
365    #[tokio::test]
366    async fn test_walk_top_down_dir() -> Result<()> {
367        let _ = tracing_subscriber::fmt().with_test_writer().try_init();
368
369        let mut iil = ImmutableIndexLayer::default();
370        for i in [
371            "dataset/stateful/ontime_2007_200.csv",
372            "dataset/stateful/ontime_2008_200.csv",
373            "dataset/stateful/ontime_2009_200.csv",
374        ] {
375            iil.insert(i.to_string())
376        }
377
378        let op = HttpConfig::from_iter({
379            let mut map = HashMap::new();
380            map.insert("endpoint".to_string(), "https://xuanwo.io".to_string());
381            map
382        })
383        .and_then(Operator::from_config)?
384        .layer(LoggingLayer::default())
385        .layer(iil)
386        .finish();
387
388        let mut ds = op.lister_with("/").recursive(true).await?;
389
390        let mut map = HashMap::new();
391        let mut set = HashSet::new();
392        while let Some(entry) = ds.try_next().await? {
393            assert!(
394                set.insert(entry.path().to_string()),
395                "duplicated value: {}",
396                entry.path()
397            );
398            map.insert(entry.path().to_string(), entry.metadata().mode());
399        }
400
401        debug!("current files: {map:?}");
402
403        assert_eq!(map["dataset/stateful/ontime_2007_200.csv"], EntryMode::FILE);
404        assert_eq!(map["dataset/stateful/ontime_2008_200.csv"], EntryMode::FILE);
405        assert_eq!(map["dataset/stateful/ontime_2009_200.csv"], EntryMode::FILE);
406        Ok(())
407    }
408}
```
