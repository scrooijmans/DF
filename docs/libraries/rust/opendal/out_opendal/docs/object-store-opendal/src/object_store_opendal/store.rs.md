# 

object_store_opendal/

store.rs

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
18use std::fmt::{self, Debug, Display, Formatter};
19use std::future::IntoFuture;
20use std::io;
21use std::sync::Arc;
22
23use crate::utils::*;
24use crate::{datetime_to_timestamp, timestamp_to_datetime};
25use async_trait::async_trait;
26use futures::FutureExt;
27use futures::StreamExt;
28use futures::TryStreamExt;
29use futures::stream::BoxStream;
30use object_store::ListResult;
31use object_store::MultipartUpload;
32use object_store::ObjectMeta;
33use object_store::ObjectStore;
34use object_store::PutMultipartOptions;
35use object_store::PutOptions;
36use object_store::PutPayload;
37use object_store::PutResult;
38use object_store::path::Path;
39use object_store::{GetOptions, UploadPart};
40use object_store::{GetRange, GetResultPayload};
41use object_store::{GetResult, PutMode};
42use opendal::Buffer;
43use opendal::Writer;
44use opendal::options::CopyOptions;
45use opendal::raw::percent_decode_path;
46use opendal::{Operator, OperatorInfo};
47use std::collections::HashMap;
48use tokio::sync::{Mutex, Notify};
49
50/// OpendalStore implements ObjectStore trait by using opendal.
51///
52/// This allows users to use opendal as an object store without extra cost.
53///
54/// Visit [`opendal::services`] for more information about supported services.
55///
56/// ```no_run
57/// use std::sync::Arc;
58///
59/// use bytes::Bytes;
60/// use object_store::path::Path;
61/// use object_store::ObjectStore;
62/// use object_store_opendal::OpendalStore;
63/// use opendal::services::S3;
64/// use opendal::{Builder, Operator};
65///
66/// #[tokio::main]
67/// async fn main() {
68///    let builder = S3::default()
69///     .access_key_id("my_access_key")
70///     .secret_access_key("my_secret_key")
71///     .endpoint("my_endpoint")
72///     .region("my_region");
73///
74///     // Create a new operator
75///     let operator = Operator::new(builder).unwrap().finish();
76///
77///     // Create a new object store
78///     let object_store = Arc::new(OpendalStore::new(operator));
79///
80///     let path = Path::from("data/nested/test.txt");
81///     let bytes = Bytes::from_static(b"hello, world! I am nested.");
82///
83///     object_store.put(&path, bytes.clone().into()).await.unwrap();
84///
85///     let content = object_store
86///         .get(&path)
87///         .await
88///         .unwrap()
89///         .bytes()
90///         .await
91///         .unwrap();
92///
93///     assert_eq!(content, bytes);
94/// }
95/// ```
96#[derive(Clone)]
97pub struct OpendalStore {
98    info: Arc<OperatorInfo>,
99    inner: Operator,
100}
101
102impl OpendalStore {
103    /// Create OpendalStore by given Operator.
104    pub fn new(op: Operator) -> Self {
105        Self {
106            info: op.info().into(),
107            inner: op,
108        }
109    }
110
111    /// Get the Operator info.
112    pub fn info(&self) -> &OperatorInfo {
113        self.info.as_ref()
114    }
115
116    /// Copy a file from one location to another
117    async fn copy_request(
118        &self,
119        from: &Path,
120        to: &Path,
121        if_not_exists: bool,
122    ) -> object_store::Result<()> {
123        let mut copy_options = CopyOptions::default();
124        if if_not_exists {
125            copy_options.if_not_exists = true;
126        }
127
128        // Perform the copy operation
129        self.inner
130            .copy_options(
131                &percent_decode_path(from.as_ref()),
132                &percent_decode_path(to.as_ref()),
133                copy_options,
134            )
135            .into_send()
136            .await
137            .map_err(|err| {
138                if if_not_exists && err.kind() == opendal::ErrorKind::AlreadyExists {
139                    object_store::Error::AlreadyExists {
140                        path: to.to_string(),
141                        source: Box::new(err),
142                    }
143                } else {
144                    format_object_store_error(err, from.as_ref())
145                }
146            })?;
147
148        Ok(())
149    }
150}
151
152impl Debug for OpendalStore {
153    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
154        f.debug_struct("OpendalStore")
155            .field("scheme", &self.info.scheme())
156            .field("name", &self.info.name())
157            .field("root", &self.info.root())
158            .field("capability", &self.info.full_capability())
159            .finish()
160    }
161}
162
163impl Display for OpendalStore {
164    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
165        let info = self.inner.info();
166        write!(
167            f,
168            "Opendal({}, bucket={}, root={})",
169            info.scheme(),
170            info.name(),
171            info.root()
172        )
173    }
174}
175
176impl From<Operator> for OpendalStore {
177    fn from(value: Operator) -> Self {
178        Self::new(value)
179    }
180}
181
182#[async_trait]
183impl ObjectStore for OpendalStore {
184    async fn put_opts(
185        &self,
186        location: &Path,
187        bytes: PutPayload,
188        opts: PutOptions,
189    ) -> object_store::Result<PutResult> {
190        let decoded_location = percent_decode_path(location.as_ref());
191        let mut future_write = self
192            .inner
193            .write_with(&decoded_location, Buffer::from_iter(bytes));
194        let opts_mode = opts.mode.clone();
195        match opts.mode {
196            PutMode::Overwrite => {}
197            PutMode::Create => {
198                future_write = future_write.if_not_exists(true);
199            }
200            PutMode::Update(update_version) => {
201                let Some(etag) = update_version.e_tag else {
202                    Err(object_store::Error::NotSupported {
203                        source: Box::new(opendal::Error::new(
204                            opendal::ErrorKind::Unsupported,
205                            "etag is required for conditional put",
206                        )),
207                    })?
208                };
209                future_write = future_write.if_match(etag.as_str());
210            }
211        }
212        let rp = future_write.into_send().await.map_err(|err| {
213            match format_object_store_error(err, location.as_ref()) {
214                object_store::Error::Precondition { path, source }
215                    if opts_mode == PutMode::Create =>
216                {
217                    object_store::Error::AlreadyExists { path, source }
218                }
219                e => e,
220            }
221        })?;
222
223        let e_tag = rp.etag().map(|s| s.to_string());
224        let version = rp.version().map(|s| s.to_string());
225
226        Ok(PutResult { e_tag, version })
227    }
228
229    async fn put_multipart(
230        &self,
231        location: &Path,
232    ) -> object_store::Result<Box<dyn MultipartUpload>> {
233        let decoded_location = percent_decode_path(location.as_ref());
234        let writer = self
235            .inner
236            .writer_with(&decoded_location)
237            .concurrent(8)
238            .into_send()
239            .await
240            .map_err(|err| format_object_store_error(err, location.as_ref()))?;
241        let upload = OpendalMultipartUpload::new(writer, location.clone());
242
243        Ok(Box::new(upload))
244    }
245
246    async fn put_multipart_opts(
247        &self,
248        location: &Path,
249        opts: PutMultipartOptions,
250    ) -> object_store::Result<Box<dyn MultipartUpload>> {
251        const DEFAULT_CONCURRENT: usize = 8;
252
253        let mut options = opendal::options::WriteOptions {
254            concurrent: DEFAULT_CONCURRENT,
255            ..Default::default()
256        };
257
258        // Collect user metadata separately to handle multiple entries
259        let mut user_metadata = HashMap::new();
260
261        // Handle attributes if provided
262        for (key, value) in opts.attributes.iter() {
263            match key {
264                object_store::Attribute::CacheControl => {
265                    options.cache_control = Some(value.to_string());
266                }
267                object_store::Attribute::ContentDisposition => {
268                    options.content_disposition = Some(value.to_string());
269                }
270                object_store::Attribute::ContentEncoding => {
271                    options.content_encoding = Some(value.to_string());
272                }
273                object_store::Attribute::ContentLanguage => {
274                    // no support
275                    continue;
276                }
277                object_store::Attribute::ContentType => {
278                    options.content_type = Some(value.to_string());
279                }
280                object_store::Attribute::Metadata(k) => {
281                    user_metadata.insert(k.to_string(), value.to_string());
282                }
283                _ => {}
284            }
285        }
286
287        // Apply user metadata if any entries were collected
288        if !user_metadata.is_empty() {
289            options.user_metadata = Some(user_metadata);
290        }
291
292        let decoded_location = percent_decode_path(location.as_ref());
293        let writer = self
294            .inner
295            .writer_options(&decoded_location, options)
296            .into_send()
297            .await
298            .map_err(|err| format_object_store_error(err, location.as_ref()))?;
299        let upload = OpendalMultipartUpload::new(writer, location.clone());
300
301        Ok(Box::new(upload))
302    }
303
304    async fn get_opts(
305        &self,
306        location: &Path,
307        options: GetOptions,
308    ) -> object_store::Result<GetResult> {
309        let raw_location = percent_decode_path(location.as_ref());
310        let meta = {
311            let mut s = self.inner.stat_with(&raw_location);
312            if let Some(version) = &options.version {
313                s = s.version(version.as_str())
314            }
315            if let Some(if_match) = &options.if_match {
316                s = s.if_match(if_match.as_str());
317            }
318            if let Some(if_none_match) = &options.if_none_match {
319                s = s.if_none_match(if_none_match.as_str());
320            }
321            if let Some(if_modified_since) =
322                options.if_modified_since.and_then(datetime_to_timestamp)
323            {
324                s = s.if_modified_since(if_modified_since);
325            }
326            if let Some(if_unmodified_since) =
327                options.if_unmodified_since.and_then(datetime_to_timestamp)
328            {
329                s = s.if_unmodified_since(if_unmodified_since);
330            }
331            s.into_send()
332                .await
333                .map_err(|err| format_object_store_error(err, location.as_ref()))?
334        };
335
336        // Convert user defined metadata from OpenDAL to object_store attributes
337        let mut attributes = object_store::Attributes::new();
338        if let Some(user_meta) = meta.user_metadata() {
339            for (key, value) in user_meta {
340                attributes.insert(
341                    object_store::Attribute::Metadata(key.clone().into()),
342                    value.clone().into(),
343                );
344            }
345        }
346
347        let meta = ObjectMeta {
348            location: location.clone(),
349            last_modified: meta
350                .last_modified()
351                .and_then(timestamp_to_datetime)
352                .unwrap_or_default(),
353            size: meta.content_length(),
354            e_tag: meta.etag().map(|x| x.to_string()),
355            version: meta.version().map(|x| x.to_string()),
356        };
357
358        if options.head {
359            return Ok(GetResult {
360                payload: GetResultPayload::Stream(Box::pin(futures::stream::empty())),
361                range: 0..0,
362                meta,
363                attributes,
364            });
365        }
366
367        let reader = {
368            let mut r = self.inner.reader_with(raw_location.as_ref());
369            if let Some(version) = options.version {
370                r = r.version(version.as_str());
371            }
372            if let Some(if_match) = options.if_match {
373                r = r.if_match(if_match.as_str());
374            }
375            if let Some(if_none_match) = options.if_none_match {
376                r = r.if_none_match(if_none_match.as_str());
377            }
378            if let Some(if_modified_since) =
379                options.if_modified_since.and_then(datetime_to_timestamp)
380            {
381                r = r.if_modified_since(if_modified_since);
382            }
383            if let Some(if_unmodified_since) =
384                options.if_unmodified_since.and_then(datetime_to_timestamp)
385            {
386                r = r.if_unmodified_since(if_unmodified_since);
387            }
388            r.into_send()
389                .await
390                .map_err(|err| format_object_store_error(err, location.as_ref()))?
391        };
392
393        let read_range = match options.range {
394            Some(GetRange::Bounded(r)) => {
395                if r.start >= r.end || r.start >= meta.size {
396                    0..0
397                } else {
398                    let end = r.end.min(meta.size);
399                    r.start..end
400                }
401            }
402            Some(GetRange::Offset(r)) => {
403                if r < meta.size {
404                    r..meta.size
405                } else {
406                    0..0
407                }
408            }
409            Some(GetRange::Suffix(r)) if r < meta.size => (meta.size - r)..meta.size,
410            _ => 0..meta.size,
411        };
412
413        let stream = reader
414            .into_bytes_stream(read_range.start..read_range.end)
415            .into_send()
416            .await
417            .map_err(|err| format_object_store_error(err, location.as_ref()))?
418            .into_send()
419            .map_err(|err: io::Error| object_store::Error::Generic {
420                store: "IoError",
421                source: Box::new(err),
422            });
423
424        Ok(GetResult {
425            payload: GetResultPayload::Stream(Box::pin(stream)),
426            range: read_range.start..read_range.end,
427            meta,
428            attributes,
429        })
430    }
431
432    async fn delete(&self, location: &Path) -> object_store::Result<()> {
433        let decoded_location = percent_decode_path(location.as_ref());
434        self.inner
435            .delete(&decoded_location)
436            .into_send()
437            .await
438            .map_err(|err| format_object_store_error(err, location.as_ref()))?;
439
440        Ok(())
441    }
442
443    fn list(&self, prefix: Option<&Path>) -> BoxStream<'static, object_store::Result<ObjectMeta>> {
444        // object_store `Path` always removes trailing slash
445        // need to add it back
446        let path = prefix.map_or("".into(), |x| {
447            format!("{}/", percent_decode_path(x.as_ref()))
448        });
449
450        let this = self.clone();
451        let fut = async move {
452            let stream = this
453                .inner
454                .lister_with(&path)
455                .recursive(true)
456                .await
457                .map_err(|err| format_object_store_error(err, &path))?;
458
459            let stream = stream.then(|res| async {
460                let entry = res.map_err(|err| format_object_store_error(err, ""))?;
461                let meta = entry.metadata();
462
463                Ok(format_object_meta(entry.path(), meta))
464            });
465            Ok::<_, object_store::Error>(stream)
466        };
467
468        fut.into_stream().try_flatten().into_send().boxed()
469    }
470
471    fn list_with_offset(
472        &self,
473        prefix: Option<&Path>,
474        offset: &Path,
475    ) -> BoxStream<'static, object_store::Result<ObjectMeta>> {
476        let path = prefix.map_or("".into(), |x| {
477            format!("{}/", percent_decode_path(x.as_ref()))
478        });
479        let offset = offset.clone();
480
481        // clone self for 'static lifetime
482        // clone self is cheap
483        let this = self.clone();
484
485        let fut = async move {
486            let list_with_start_after = this.inner.info().full_capability().list_with_start_after;
487            let mut fut = this.inner.lister_with(&path).recursive(true);
488
489            // Use native start_after support if possible.
490            if list_with_start_after {
491                fut = fut.start_after(offset.as_ref());
492            }
493
494            let lister = fut
495                .await
496                .map_err(|err| format_object_store_error(err, &path))?
497                .then(move |entry| {
498                    let path = path.clone();
499                    let this = this.clone();
500                    async move {
501                        let entry = entry.map_err(|err| format_object_store_error(err, &path))?;
502                        let (path, metadata) = entry.into_parts();
503
504                        // If it's a dir or last_modified is present, we can use it directly.
505                        if metadata.is_dir() || metadata.last_modified().is_some() {
506                            let object_meta = format_object_meta(&path, &metadata);
507                            return Ok(object_meta);
508                        }
509
510                        let metadata = this
511                            .inner
512                            .stat(&path)
513                            .await
514                            .map_err(|err| format_object_store_error(err, &path))?;
515                        let object_meta = format_object_meta(&path, &metadata);
516                        Ok::<_, object_store::Error>(object_meta)
517                    }
518                })
519                .into_send()
520                .boxed();
521
522            let stream = if list_with_start_after {
523                lister
524            } else {
525                lister
526                    .try_filter(move |entry| futures::future::ready(entry.location > offset))
527                    .into_send()
528                    .boxed()
529            };
530
531            Ok::<_, object_store::Error>(stream)
532        };
533
534        fut.into_stream().into_send().try_flatten().boxed()
535    }
536
537    async fn list_with_delimiter(&self, prefix: Option<&Path>) -> object_store::Result<ListResult> {
538        let path = prefix.map_or("".into(), |x| {
539            format!("{}/", percent_decode_path(x.as_ref()))
540        });
541        let mut stream = self
542            .inner
543            .lister_with(&path)
544            .into_future()
545            .into_send()
546            .await
547            .map_err(|err| format_object_store_error(err, &path))?
548            .into_send();
549
550        let mut common_prefixes = Vec::new();
551        let mut objects = Vec::new();
552
553        while let Some(res) = stream.next().into_send().await {
554            let entry = res.map_err(|err| format_object_store_error(err, ""))?;
555            let meta = entry.metadata();
556
557            if meta.is_dir() {
558                common_prefixes.push(entry.path().into());
559            } else if meta.last_modified().is_some() {
560                objects.push(format_object_meta(entry.path(), meta));
561            } else {
562                let meta = self
563                    .inner
564                    .stat(entry.path())
565                    .into_send()
566                    .await
567                    .map_err(|err| format_object_store_error(err, entry.path()))?;
568                objects.push(format_object_meta(entry.path(), &meta));
569            }
570        }
571
572        Ok(ListResult {
573            common_prefixes,
574            objects,
575        })
576    }
577
578    async fn copy(&self, from: &Path, to: &Path) -> object_store::Result<()> {
579        self.copy_request(from, to, false).await
580    }
581
582    async fn copy_if_not_exists(&self, from: &Path, to: &Path) -> object_store::Result<()> {
583        self.copy_request(from, to, true).await
584    }
585
586    async fn rename(&self, from: &Path, to: &Path) -> object_store::Result<()> {
587        self.inner
588            .rename(
589                &percent_decode_path(from.as_ref()),
590                &percent_decode_path(to.as_ref()),
591            )
592            .into_send()
593            .await
594            .map_err(|err| format_object_store_error(err, from.as_ref()))?;
595
596        Ok(())
597    }
598}
599
600/// `MultipartUpload`'s impl based on `Writer` in opendal
601///
602/// # Notes
603///
604/// OpenDAL writer can handle concurrent internally we don't generate real `UploadPart` like existing
605/// implementation do. Instead, we just write the part and notify the next task to be written.
606///
607/// The lock here doesn't really involve the write process, it's just for the notify mechanism.
608struct OpendalMultipartUpload {
609    writer: Arc<Mutex<Writer>>,
610    location: Path,
611    next_notify: Option<Arc<Notify>>,
612}
613
614impl OpendalMultipartUpload {
615    fn new(writer: Writer, location: Path) -> Self {
616        Self {
617            writer: Arc::new(Mutex::new(writer)),
618            location,
619            next_notify: None,
620        }
621    }
622}
623
624#[async_trait]
625impl MultipartUpload for OpendalMultipartUpload {
626    fn put_part(&mut self, data: PutPayload) -> UploadPart {
627        let writer = self.writer.clone();
628        let location = self.location.clone();
629
630        // Generate next notify which will be notified after the current part is written.
631        let next_notify = Arc::new(Notify::new());
632        // Fetch the notify for current part to wait for it to be written.
633        let current_notify = self.next_notify.replace(next_notify.clone());
634
635        async move {
636            // current_notify == None means that it's the first part, we don't need to wait.
637            if let Some(notify) = current_notify {
638                // Wait for the previous part to be written
639                notify.notified().await;
640            }
641
642            let mut writer = writer.lock().await;
643            let result = writer
644                .write(Buffer::from_iter(data.into_iter()))
645                .await
646                .map_err(|err| format_object_store_error(err, location.as_ref()));
647
648            // Notify the next part to be written
649            next_notify.notify_one();
650
651            result
652        }
653        .into_send()
654        .boxed()
655    }
656
657    async fn complete(&mut self) -> object_store::Result<PutResult> {
658        let mut writer = self.writer.lock().await;
659        let metadata = writer
660            .close()
661            .into_send()
662            .await
663            .map_err(|err| format_object_store_error(err, self.location.as_ref()))?;
664
665        let e_tag = metadata.etag().map(|s| s.to_string());
666        let version = metadata.version().map(|s| s.to_string());
667
668        Ok(PutResult { e_tag, version })
669    }
670
671    async fn abort(&mut self) -> object_store::Result<()> {
672        let mut writer = self.writer.lock().await;
673        writer
674            .abort()
675            .into_send()
676            .await
677            .map_err(|err| format_object_store_error(err, self.location.as_ref()))
678    }
679}
680
681impl Debug for OpendalMultipartUpload {
682    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
683        f.debug_struct("OpendalMultipartUpload")
684            .field("location", &self.location)
685            .finish()
686    }
687}
688
689#[cfg(test)]
690mod tests {
691    use bytes::Bytes;
692    use object_store::path::Path;
693    use object_store::{ObjectStore, WriteMultipart};
694    use opendal::services;
695    use rand::prelude::*;
696    use std::sync::Arc;
697
698    use super::*;
699
700    async fn create_test_object_store() -> Arc<dyn ObjectStore> {
701        let op = Operator::new(services::Memory::default()).unwrap().finish();
702        let object_store = Arc::new(OpendalStore::new(op));
703
704        let path: Path = "data/test.txt".into();
705        let bytes = Bytes::from_static(b"hello, world!");
706        object_store.put(&path, bytes.into()).await.unwrap();
707
708        let path: Path = "data/nested/test.txt".into();
709        let bytes = Bytes::from_static(b"hello, world! I am nested.");
710        object_store.put(&path, bytes.into()).await.unwrap();
711
712        object_store
713    }
714
715    #[tokio::test]
716    async fn test_basic() {
717        let op = Operator::new(services::Memory::default()).unwrap().finish();
718        let object_store: Arc<dyn ObjectStore> = Arc::new(OpendalStore::new(op));
719
720        // Retrieve a specific file
721        let path: Path = "data/test.txt".into();
722
723        let bytes = Bytes::from_static(b"hello, world!");
724        object_store.put(&path, bytes.clone().into()).await.unwrap();
725
726        let meta = object_store.head(&path).await.unwrap();
727
728        assert_eq!(meta.size, 13);
729
730        assert_eq!(
731            object_store
732                .get(&path)
733                .await
734                .unwrap()
735                .bytes()
736                .await
737                .unwrap(),
738            bytes
739        );
740    }
741
742    #[tokio::test]
743    async fn test_put_multipart() {
744        let op = Operator::new(services::Memory::default()).unwrap().finish();
745        let object_store: Arc<dyn ObjectStore> = Arc::new(OpendalStore::new(op));
746
747        let mut rng = thread_rng();
748
749        // Case complete
750        let path: Path = "data/test_complete.txt".into();
751        let upload = object_store.put_multipart(&path).await.unwrap();
752
753        let mut write = WriteMultipart::new(upload);
754
755        let mut all_bytes = vec![];
756        let round = rng.gen_range(1..=1024);
757        for _ in 0..round {
758            let size = rng.gen_range(1..=1024);
759            let mut bytes = vec![0; size];
760            rng.fill_bytes(&mut bytes);
761
762            all_bytes.extend_from_slice(&bytes);
763            write.put(bytes.into());
764        }
765
766        let _ = write.finish().await.unwrap();
767
768        let meta = object_store.head(&path).await.unwrap();
769
770        assert_eq!(meta.size, all_bytes.len() as u64);
771
772        assert_eq!(
773            object_store
774                .get(&path)
775                .await
776                .unwrap()
777                .bytes()
778                .await
779                .unwrap(),
780            Bytes::from(all_bytes)
781        );
782
783        // Case abort
784        let path: Path = "data/test_abort.txt".into();
785        let mut upload = object_store.put_multipart(&path).await.unwrap();
786        upload.put_part(vec![1; 1024].into()).await.unwrap();
787        upload.abort().await.unwrap();
788
789        let res = object_store.head(&path).await;
790        let err = res.unwrap_err();
791
792        assert!(matches!(err, object_store::Error::NotFound { .. }))
793    }
794
795    #[tokio::test]
796    async fn test_list() {
797        let object_store = create_test_object_store().await;
798        let path: Path = "data/".into();
799        let results = object_store.list(Some(&path)).collect::<Vec<_>>().await;
800        assert_eq!(results.len(), 2);
801        let mut locations = results
802            .iter()
803            .map(|x| x.as_ref().unwrap().location.as_ref())
804            .collect::<Vec<_>>();
805
806        let expected_files = vec![
807            (
808                "data/nested/test.txt",
809                Bytes::from_static(b"hello, world! I am nested."),
810            ),
811            ("data/test.txt", Bytes::from_static(b"hello, world!")),
812        ];
813
814        let expected_locations = expected_files.iter().map(|x| x.0).collect::<Vec<&str>>();
815
816        locations.sort();
817        assert_eq!(locations, expected_locations);
818
819        for (location, bytes) in expected_files {
820            let path: Path = location.into();
821            assert_eq!(
822                object_store
823                    .get(&path)
824                    .await
825                    .unwrap()
826                    .bytes()
827                    .await
828                    .unwrap(),
829                bytes
830            );
831        }
832    }
833
834    #[tokio::test]
835    async fn test_list_with_delimiter() {
836        let object_store = create_test_object_store().await;
837        let path: Path = "data/".into();
838        let result = object_store.list_with_delimiter(Some(&path)).await.unwrap();
839        assert_eq!(result.objects.len(), 1);
840        assert_eq!(result.common_prefixes.len(), 1);
841        assert_eq!(result.objects[0].location.as_ref(), "data/test.txt");
842        assert_eq!(result.common_prefixes[0].as_ref(), "data/nested");
843    }
844
845    #[tokio::test]
846    async fn test_list_with_offset() {
847        let object_store = create_test_object_store().await;
848        let path: Path = "data/".into();
849        let offset: Path = "data/nested/test.txt".into();
850        let result = object_store
851            .list_with_offset(Some(&path), &offset)
852            .collect::<Vec<_>>()
853            .await;
854        assert_eq!(result.len(), 1);
855        assert_eq!(
856            result[0].as_ref().unwrap().location.as_ref(),
857            "data/test.txt"
858        );
859    }
860}
```
