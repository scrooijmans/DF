# 

object_store_opendal/service/

mod.rs

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
20use std::sync::Arc;
21
22use object_store::ObjectStore;
23use object_store::path::Path as ObjectStorePath;
24use opendal::Error;
25use opendal::ErrorKind;
26use opendal::raw::oio::BatchDeleter;
27use opendal::raw::oio::MultipartWriter;
28use opendal::raw::*;
29use opendal::*;
30
31mod core;
32mod deleter;
33mod error;
34mod lister;
35mod reader;
36mod writer;
37
38use deleter::ObjectStoreDeleter;
39use error::parse_error;
40use lister::ObjectStoreLister;
41use reader::ObjectStoreReader;
42use writer::ObjectStoreWriter;
43
44use crate::service::core::format_metadata as parse_metadata;
45use crate::service::core::parse_op_stat;
46
47/// ObjectStore backend builder
48#[derive(Default)]
49pub struct ObjectStoreBuilder {
50    store: Option<Arc<dyn ObjectStore + 'static>>,
51}
52
53impl Debug for ObjectStoreBuilder {
54    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
55        let mut d = f.debug_struct("ObjectStoreBuilder");
56        d.finish_non_exhaustive()
57    }
58}
59
60impl ObjectStoreBuilder {
61    /// Set the object store instance
62    pub fn new(store: Arc<dyn ObjectStore + 'static>) -> Self {
63        Self { store: Some(store) }
64    }
65}
66
67impl Builder for ObjectStoreBuilder {
68    type Config = ();
69
70    fn build(self) -> Result<impl Access> {
71        let store = self.store.ok_or_else(|| {
72            Error::new(ErrorKind::ConfigInvalid, "object store is required")
73                .with_context("service", Scheme::Custom("object_store"))
74        })?;
75
76        Ok(ObjectStoreService { store })
77    }
78}
79
80/// ObjectStore backend
81pub struct ObjectStoreService {
82    store: Arc<dyn ObjectStore + 'static>,
83}
84
85impl Debug for ObjectStoreService {
86    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
87        let mut d = f.debug_struct("ObjectStoreBackend");
88        d.finish_non_exhaustive()
89    }
90}
91
92impl Access for ObjectStoreService {
93    type Reader = ObjectStoreReader;
94    type Writer = MultipartWriter<ObjectStoreWriter>;
95    type Lister = ObjectStoreLister;
96    type Deleter = BatchDeleter<ObjectStoreDeleter>;
97
98    fn info(&self) -> Arc<AccessorInfo> {
99        let info = AccessorInfo::default();
100        info.set_scheme("object_store")
101            .set_root("/")
102            .set_name("object_store")
103            .set_native_capability(Capability {
104                stat: true,
105                stat_with_if_match: true,
106                stat_with_if_unmodified_since: true,
107                read: true,
108                write: true,
109                delete: true,
110                list: true,
111                list_with_limit: true,
112                list_with_start_after: true,
113                delete_with_version: false,
114                ..Default::default()
115            });
116        Arc::new(info)
117    }
118
119    async fn stat(&self, path: &str, args: OpStat) -> Result<RpStat> {
120        let path = ObjectStorePath::from(path);
121        let opts = parse_op_stat(&args)?;
122        let result = self
123            .store
124            .get_opts(&path, opts)
125            .await
126            .map_err(parse_error)?;
127        let metadata = parse_metadata(&result.meta);
128        Ok(RpStat::new(metadata))
129    }
130
131    async fn read(&self, path: &str, args: OpRead) -> Result<(RpRead, Self::Reader)> {
132        let reader = ObjectStoreReader::new(self.store.clone(), path, args).await?;
133        Ok((reader.rp(), reader))
134    }
135
136    async fn write(&self, path: &str, args: OpWrite) -> Result<(RpWrite, Self::Writer)> {
137        let writer = ObjectStoreWriter::new(self.store.clone(), path, args);
138        Ok((
139            RpWrite::default(),
140            MultipartWriter::new(self.info(), writer, 10),
141        ))
142    }
143
144    async fn delete(&self) -> Result<(RpDelete, Self::Deleter)> {
145        let deleter = BatchDeleter::new(ObjectStoreDeleter::new(self.store.clone()));
146        Ok((RpDelete::default(), deleter))
147    }
148
149    async fn list(&self, path: &str, args: OpList) -> Result<(RpList, Self::Lister)> {
150        let lister = ObjectStoreLister::new(self.store.clone(), path, args).await?;
151        Ok((RpList::default(), lister))
152    }
153}
154
155#[cfg(test)]
156mod tests {
157    use super::*;
158    use object_store::memory::InMemory;
159    use opendal::Buffer;
160    use opendal::raw::oio::{Delete, List, Read, Write};
161
162    #[tokio::test]
163    async fn test_object_store_backend_builder() {
164        let store: Arc<dyn ObjectStore> = Arc::new(InMemory::new());
165        let builder = ObjectStoreBuilder::new(store);
166
167        let backend = builder.build().expect("build should succeed");
168        assert!(backend.info().scheme() == "object_store");
169    }
170
171    #[tokio::test]
172    async fn test_object_store_backend_info() {
173        let store: Arc<dyn ObjectStore> = Arc::new(InMemory::new());
174        let backend = ObjectStoreBuilder::new(store)
175            .build()
176            .expect("build should succeed");
177
178        let info = backend.info();
179        assert_eq!(info.scheme(), "object_store");
180        assert_eq!(info.name(), "object_store".into());
181        assert_eq!(info.root(), "/".into());
182
183        let cap = info.native_capability();
184        assert!(cap.stat);
185        assert!(cap.read);
186        assert!(cap.write);
187        assert!(cap.delete);
188        assert!(cap.list);
189    }
190
191    #[tokio::test]
192    async fn test_object_store_backend_basic_operations() {
193        let store: Arc<dyn ObjectStore> = Arc::new(InMemory::new());
194        let backend = ObjectStoreBuilder::new(store.clone())
195            .build()
196            .expect("build should succeed");
197
198        let path = "test_file.txt";
199        let content = b"Hello, world!";
200
201        // Test write
202        let (_, mut writer) = backend
203            .write(path, OpWrite::default())
204            .await
205            .expect("write should succeed");
206
207        writer
208            .write(Buffer::from(&content[..]))
209            .await
210            .expect("write content should succeed");
211        writer.close().await.expect("close should succeed");
212
213        // Test stat
214        let stat_result = backend
215            .stat(path, OpStat::default())
216            .await
217            .expect("stat should succeed");
218
219        assert_eq!(
220            stat_result.into_metadata().content_length(),
221            content.len() as u64
222        );
223
224        // Test read
225        let (_, mut reader) = backend
226            .read(path, OpRead::default())
227            .await
228            .expect("read should succeed");
229
230        let buf = reader.read().await.expect("read should succeed");
231        assert_eq!(buf.to_vec(), content);
232    }
233
234    #[tokio::test]
235    async fn test_object_store_backend_multipart_upload() {
236        let store: Arc<dyn ObjectStore> = Arc::new(InMemory::new());
237        let backend = ObjectStoreBuilder::new(store.clone())
238            .build()
239            .expect("build should succeed");
240
241        let path = "test_file.txt";
242        let content =
243            b"Hello, multipart upload! This is a test content for multipart upload functionality.";
244        let content_len = content.len();
245
246        // Test multipart upload with multiple chunks
247        let (_, mut writer) = backend
248            .write(path, OpWrite::default())
249            .await
250            .expect("write should succeed");
251
252        // Write content in chunks to simulate multipart upload
253        let chunk_size = 20;
254        for chunk in content.chunks(chunk_size) {
255            writer
256                .write(Buffer::from(chunk))
257                .await
258                .expect("write chunk should succeed");
259        }
260
261        writer.close().await.expect("close should succeed");
262
263        // Verify the uploaded file
264        let stat_result = backend
265            .stat(path, OpStat::default())
266            .await
267            .expect("stat should succeed");
268
269        assert_eq!(
270            stat_result.into_metadata().content_length(),
271            content_len as u64
272        );
273
274        // Read back and verify content
275        let (_, mut reader) = backend
276            .read(path, OpRead::default())
277            .await
278            .expect("read should succeed");
279
280        let buf = reader.read().await.expect("read should succeed");
281        assert_eq!(buf.to_vec(), content);
282    }
283
284    #[tokio::test]
285    async fn test_object_store_backend_list() {
286        let store: Arc<dyn ObjectStore> = Arc::new(InMemory::new());
287        let backend = ObjectStoreBuilder::new(store.clone())
288            .build()
289            .expect("build should succeed");
290
291        // Create multiple files
292        let files = vec![
293            ("dir1/file1.txt", b"content1"),
294            ("dir1/file2.txt", b"content2"),
295            ("dir2/file3.txt", b"content3"),
296        ];
297
298        for (path, content) in &files {
299            let (_, mut writer) = backend
300                .write(path, OpWrite::default())
301                .await
302                .expect("write should succeed");
303            writer
304                .write(Buffer::from(&content[..]))
305                .await
306                .expect("write content should succeed");
307            writer.close().await.expect("close should succeed");
308        }
309
310        // List directory
311        let (_, mut lister) = backend
312            .list("dir1/", OpList::default())
313            .await
314            .expect("list should succeed");
315
316        let mut entries = Vec::new();
317        while let Some(entry) = lister.next().await.expect("next should succeed") {
318            entries.push(entry);
319        }
320
321        assert_eq!(entries.len(), 2);
322        assert!(entries.iter().any(|e| e.path() == "dir1/file1.txt"));
323        assert!(entries.iter().any(|e| e.path() == "dir1/file2.txt"));
324    }
325
326    #[tokio::test]
327    async fn test_object_store_backend_delete() {
328        let store: Arc<dyn ObjectStore> = Arc::new(InMemory::new());
329        let backend = ObjectStoreBuilder::new(store)
330            .build()
331            .expect("build should succeed");
332
333        let path = "test_delete.txt";
334        let content = b"To be deleted";
335
336        // Write file
337        let (_, mut writer) = backend
338            .write(path, OpWrite::default())
339            .await
340            .expect("write should succeed");
341        writer
342            .write(Buffer::from(&content[..]))
343            .await
344            .expect("write content should succeed");
345        writer.close().await.expect("close should succeed");
346
347        // Verify file exists
348        backend
349            .stat(path, OpStat::default())
350            .await
351            .expect("file should exist");
352
353        // Delete file
354        let (_, mut deleter) = backend.delete().await.expect("delete should succeed");
355        deleter
356            .delete(path, OpDelete::default())
357            .expect("delete should succeed");
358        deleter.flush().await.expect("flush should succeed");
359
360        // Verify file is deleted
361        let result = backend.stat(path, OpStat::default()).await;
362        assert!(result.is_err());
363    }
364
365    #[tokio::test]
366    async fn test_object_store_backend_error_handling() {
367        let store: Arc<dyn ObjectStore> = Arc::new(InMemory::new());
368        let backend = ObjectStoreBuilder::new(store)
369            .build()
370            .expect("build should succeed");
371
372        // Test stat on non-existent file
373        let result = backend.stat("non_existent.txt", OpStat::default()).await;
374        assert!(result.is_err());
375
376        // Test read on non-existent file
377        let result = backend.read("non_existent.txt", OpRead::default()).await;
378        assert!(result.is_err());
379
380        // Test list on non-existent directory
381        let result = backend.list("non_existent_dir/", OpList::default()).await;
382        // This should succeed but return empty results
383        if let Ok((_, mut lister)) = result {
384            let entry = lister.next().await.expect("next should succeed");
385            assert!(entry.is_none());
386        }
387    }
388}
```
