# 

opendal/raw/

ops.rs

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
18//! Ops provides the operation args struct like [`OpRead`] for user.
19//!
20//! By using ops, users can add more context for operation.
21
22use crate::options;
23use crate::raw::*;
24use std::collections::HashMap;
25use std::time::Duration;
26
27/// Args for `create` operation.
28///
29/// The path must be normalized.
30#[derive(Debug, Clone, Default)]
31pub struct OpCreateDir {}
32
33impl OpCreateDir {
34    /// Create a new `OpCreateDir`.
35    pub fn new() -> Self {
36        Self::default()
37    }
38}
39
40/// Args for `delete` operation.
41///
42/// The path must be normalized.
43#[derive(Debug, Clone, Default, Eq, Hash, PartialEq)]
44pub struct OpDelete {
45    version: Option<String>,
46}
47
48impl OpDelete {
49    /// Create a new `OpDelete`.
50    pub fn new() -> Self {
51        Self::default()
52    }
53}
54
55impl OpDelete {
56    /// Change the version of this delete operation.
57    pub fn with_version(mut self, version: &str) -> Self {
58        self.version = Some(version.into());
59        self
60    }
61
62    /// Get the version of this delete operation.
63    pub fn version(&self) -> Option<&str> {
64        self.version.as_deref()
65    }
66}
67
68impl From<options::DeleteOptions> for OpDelete {
69    fn from(value: options::DeleteOptions) -> Self {
70        Self {
71            version: value.version,
72        }
73    }
74}
75
76/// Args for `delete` operation.
77///
78/// The path must be normalized.
79#[derive(Debug, Clone, Default)]
80pub struct OpDeleter {}
81
82impl OpDeleter {
83    /// Create a new `OpDelete`.
84    pub fn new() -> Self {
85        Self::default()
86    }
87}
88
89/// Args for `list` operation.
90#[derive(Debug, Clone, Default)]
91pub struct OpList {
92    /// The limit passed to underlying service to specify the max results
93    /// that could return per-request.
94    ///
95    /// Users could use this to control the memory usage of list operation.
96    limit: Option<usize>,
97    /// The start_after passes to underlying service to specify the specified key
98    /// to start listing from.
99    start_after: Option<String>,
100    /// The recursive is used to control whether the list operation is recursive.
101    ///
102    /// - If `false`, list operation will only list the entries under the given path.
103    /// - If `true`, list operation will list all entries that starts with given path.
104    ///
105    /// Default to `false`.
106    recursive: bool,
107    /// The version is used to control whether the object versions should be returned.
108    ///
109    /// - If `false`, list operation will not return with object versions
110    /// - If `true`, list operation will return with object versions if object versioning is supported
111    ///   by the underlying service
112    ///
113    /// Default to `false`
114    versions: bool,
115    /// The deleted is used to control whether the deleted objects should be returned.
116    ///
117    /// - If `false`, list operation will not return with deleted objects
118    /// - If `true`, list operation will return with deleted objects if object versioning is supported
119    ///   by the underlying service
120    ///
121    /// Default to `false`
122    deleted: bool,
123}
124
125impl OpList {
126    /// Create a new `OpList`.
127    pub fn new() -> Self {
128        Self::default()
129    }
130
131    /// Change the limit of this list operation.
132    pub fn with_limit(mut self, limit: usize) -> Self {
133        self.limit = Some(limit);
134        self
135    }
136
137    /// Get the limit of list operation.
138    pub fn limit(&self) -> Option<usize> {
139        self.limit
140    }
141
142    /// Change the start_after of this list operation.
143    pub fn with_start_after(mut self, start_after: &str) -> Self {
144        self.start_after = Some(start_after.into());
145        self
146    }
147
148    /// Get the start_after of list operation.
149    pub fn start_after(&self) -> Option<&str> {
150        self.start_after.as_deref()
151    }
152
153    /// The recursive is used to control whether the list operation is recursive.
154    ///
155    /// - If `false`, list operation will only list the entries under the given path.
156    /// - If `true`, list operation will list all entries that starts with given path.
157    ///
158    /// Default to `false`.
159    pub fn with_recursive(mut self, recursive: bool) -> Self {
160        self.recursive = recursive;
161        self
162    }
163
164    /// Get the current recursive.
165    pub fn recursive(&self) -> bool {
166        self.recursive
167    }
168
169    /// Change the concurrent of this list operation.
170    ///
171    /// The default concurrent is 1.
172    #[deprecated(since = "0.53.2", note = "concurrent in list is no-op")]
173    pub fn with_concurrent(self, concurrent: usize) -> Self {
174        let _ = concurrent;
175        self
176    }
177
178    /// Get the concurrent of list operation.
179    #[deprecated(since = "0.53.2", note = "concurrent in list is no-op")]
180    pub fn concurrent(&self) -> usize {
181        0
182    }
183
184    /// Change the version of this list operation
185    #[deprecated(since = "0.51.1", note = "use with_versions instead")]
186    pub fn with_version(mut self, version: bool) -> Self {
187        self.versions = version;
188        self
189    }
190
191    /// Change the version of this list operation
192    pub fn with_versions(mut self, versions: bool) -> Self {
193        self.versions = versions;
194        self
195    }
196
197    /// Get the version of this list operation
198    #[deprecated(since = "0.51.1", note = "use versions instead")]
199    pub fn version(&self) -> bool {
200        self.versions
201    }
202
203    /// Get the version of this list operation
204    pub fn versions(&self) -> bool {
205        self.versions
206    }
207
208    /// Change the deleted of this list operation
209    pub fn with_deleted(mut self, deleted: bool) -> Self {
210        self.deleted = deleted;
211        self
212    }
213
214    /// Get the deleted of this list operation
215    pub fn deleted(&self) -> bool {
216        self.deleted
217    }
218}
219
220impl From<options::ListOptions> for OpList {
221    fn from(value: options::ListOptions) -> Self {
222        Self {
223            limit: value.limit,
224            start_after: value.start_after,
225            recursive: value.recursive,
226            versions: value.versions,
227            deleted: value.deleted,
228        }
229    }
230}
231
232/// Args for `presign` operation.
233///
234/// The path must be normalized.
235#[derive(Debug, Clone)]
236pub struct OpPresign {
237    expire: Duration,
238
239    op: PresignOperation,
240}
241
242impl OpPresign {
243    /// Create a new `OpPresign`.
244    pub fn new(op: impl Into<PresignOperation>, expire: Duration) -> Self {
245        Self {
246            op: op.into(),
247            expire,
248        }
249    }
250
251    /// Get operation from op.
252    pub fn operation(&self) -> &PresignOperation {
253        &self.op
254    }
255
256    /// Get expire from op.
257    pub fn expire(&self) -> Duration {
258        self.expire
259    }
260
261    /// Consume OpPresign into (Duration, PresignOperation)
262    pub fn into_parts(self) -> (Duration, PresignOperation) {
263        (self.expire, self.op)
264    }
265}
266
267/// Presign operation used for presign.
268#[derive(Debug, Clone)]
269#[non_exhaustive]
270pub enum PresignOperation {
271    /// Presign a stat(head) operation.
272    Stat(OpStat),
273    /// Presign a read operation.
274    Read(OpRead),
275    /// Presign a write operation.
276    Write(OpWrite),
277    /// Presign a delete operation.
278    Delete(OpDelete),
279}
280
281impl From<OpStat> for PresignOperation {
282    fn from(op: OpStat) -> Self {
283        Self::Stat(op)
284    }
285}
286
287impl From<OpRead> for PresignOperation {
288    fn from(v: OpRead) -> Self {
289        Self::Read(v)
290    }
291}
292
293impl From<OpWrite> for PresignOperation {
294    fn from(v: OpWrite) -> Self {
295        Self::Write(v)
296    }
297}
298
299impl From<OpDelete> for PresignOperation {
300    fn from(v: OpDelete) -> Self {
301        Self::Delete(v)
302    }
303}
304
305/// Args for `read` operation.
306#[derive(Debug, Clone, Default)]
307pub struct OpRead {
308    range: BytesRange,
309    if_match: Option<String>,
310    if_none_match: Option<String>,
311    if_modified_since: Option<Timestamp>,
312    if_unmodified_since: Option<Timestamp>,
313    override_content_type: Option<String>,
314    override_cache_control: Option<String>,
315    override_content_disposition: Option<String>,
316    version: Option<String>,
317}
318
319impl OpRead {
320    /// Create a default `OpRead` which will read whole content of path.
321    pub fn new() -> Self {
322        Self::default()
323    }
324
325    /// Set the range of the option
326    pub fn with_range(mut self, range: BytesRange) -> Self {
327        self.range = range;
328        self
329    }
330
331    /// Get range from option
332    pub fn range(&self) -> BytesRange {
333        self.range
334    }
335
336    /// Returns a mutable range to allow updating.
337    pub(crate) fn range_mut(&mut self) -> &mut BytesRange {
338        &mut self.range
339    }
340
341    /// Sets the content-disposition header that should be sent back by the remote read operation.
342    pub fn with_override_content_disposition(mut self, content_disposition: &str) -> Self {
343        self.override_content_disposition = Some(content_disposition.into());
344        self
345    }
346
347    /// Returns the content-disposition header that should be sent back by the remote read
348    /// operation.
349    pub fn override_content_disposition(&self) -> Option<&str> {
350        self.override_content_disposition.as_deref()
351    }
352
353    /// Sets the cache-control header that should be sent back by the remote read operation.
354    pub fn with_override_cache_control(mut self, cache_control: &str) -> Self {
355        self.override_cache_control = Some(cache_control.into());
356        self
357    }
358
359    /// Returns the cache-control header that should be sent back by the remote read operation.
360    pub fn override_cache_control(&self) -> Option<&str> {
361        self.override_cache_control.as_deref()
362    }
363
364    /// Sets the content-type header that should be sent back by the remote read operation.
365    pub fn with_override_content_type(mut self, content_type: &str) -> Self {
366        self.override_content_type = Some(content_type.into());
367        self
368    }
369
370    /// Returns the content-type header that should be sent back by the remote read operation.
371    pub fn override_content_type(&self) -> Option<&str> {
372        self.override_content_type.as_deref()
373    }
374
375    /// Set the If-Match of the option
376    pub fn with_if_match(mut self, if_match: &str) -> Self {
377        self.if_match = Some(if_match.to_string());
378        self
379    }
380
381    /// Get If-Match from option
382    pub fn if_match(&self) -> Option<&str> {
383        self.if_match.as_deref()
384    }
385
386    /// Set the If-None-Match of the option
387    pub fn with_if_none_match(mut self, if_none_match: &str) -> Self {
388        self.if_none_match = Some(if_none_match.to_string());
389        self
390    }
391
392    /// Get If-None-Match from option
393    pub fn if_none_match(&self) -> Option<&str> {
394        self.if_none_match.as_deref()
395    }
396
397    /// Set the If-Modified-Since of the option
398    pub fn with_if_modified_since(mut self, v: Timestamp) -> Self {
399        self.if_modified_since = Some(v);
400        self
401    }
402
403    /// Get If-Modified-Since from option
404    pub fn if_modified_since(&self) -> Option<Timestamp> {
405        self.if_modified_since
406    }
407
408    /// Set the If-Unmodified-Since of the option
409    pub fn with_if_unmodified_since(mut self, v: Timestamp) -> Self {
410        self.if_unmodified_since = Some(v);
411        self
412    }
413
414    /// Get If-Unmodified-Since from option
415    pub fn if_unmodified_since(&self) -> Option<Timestamp> {
416        self.if_unmodified_since
417    }
418
419    /// Set the version of the option
420    pub fn with_version(mut self, version: &str) -> Self {
421        self.version = Some(version.to_string());
422        self
423    }
424
425    /// Get version from option
426    pub fn version(&self) -> Option<&str> {
427        self.version.as_deref()
428    }
429}
430
431/// Args for reader operation.
432#[derive(Debug, Clone)]
433pub struct OpReader {
434    /// The concurrent requests that reader can send.
435    concurrent: usize,
436    /// The chunk size of each request.
437    chunk: Option<usize>,
438    /// The gap size of each request.
439    gap: Option<usize>,
440    /// The maximum number of buffers that can be prefetched.
441    prefetch: usize,
442}
443
444impl Default for OpReader {
445    fn default() -> Self {
446        Self {
447            concurrent: 1,
448            chunk: None,
449            gap: None,
450            prefetch: 0,
451        }
452    }
453}
454
455impl OpReader {
456    /// Create a new `OpReader`.
457    pub fn new() -> Self {
458        Self::default()
459    }
460
461    /// Set the concurrent of the option
462    pub fn with_concurrent(mut self, concurrent: usize) -> Self {
463        self.concurrent = concurrent.max(1);
464        self
465    }
466
467    /// Get concurrent from option
468    pub fn concurrent(&self) -> usize {
469        self.concurrent
470    }
471
472    /// Set the chunk of the option
473    pub fn with_chunk(mut self, chunk: usize) -> Self {
474        self.chunk = Some(chunk.max(1));
475        self
476    }
477
478    /// Get chunk from option
479    pub fn chunk(&self) -> Option<usize> {
480        self.chunk
481    }
482
483    /// Set the gap of the option
484    pub fn with_gap(mut self, gap: usize) -> Self {
485        self.gap = Some(gap.max(1));
486        self
487    }
488
489    /// Get gap from option
490    pub fn gap(&self) -> Option<usize> {
491        self.gap
492    }
493
494    /// Set the prefetch of the option
495    pub fn with_prefetch(mut self, prefetch: usize) -> Self {
496        self.prefetch = prefetch;
497        self
498    }
499
500    /// Get prefetch from option
501    pub fn prefetch(&self) -> usize {
502        self.prefetch
503    }
504}
505
506impl From<options::ReadOptions> for (OpRead, OpReader) {
507    fn from(value: options::ReadOptions) -> Self {
508        (
509            OpRead {
510                range: value.range,
511                if_match: value.if_match,
512                if_none_match: value.if_none_match,
513                if_modified_since: value.if_modified_since,
514                if_unmodified_since: value.if_unmodified_since,
515                override_content_type: value.override_content_type,
516                override_cache_control: value.override_cache_control,
517                override_content_disposition: value.override_content_disposition,
518                version: value.version,
519            },
520            OpReader {
521                // Ensure concurrent is at least 1
522                concurrent: value.concurrent.max(1),
523                chunk: value.chunk,
524                gap: value.gap,
525                prefetch: 0,
526            },
527        )
528    }
529}
530
531impl From<options::ReaderOptions> for (OpRead, OpReader) {
532    fn from(value: options::ReaderOptions) -> Self {
533        (
534            OpRead {
535                range: BytesRange::default(),
536                if_match: value.if_match,
537                if_none_match: value.if_none_match,
538                if_modified_since: value.if_modified_since,
539                if_unmodified_since: value.if_unmodified_since,
540                override_content_type: None,
541                override_cache_control: None,
542                override_content_disposition: None,
543                version: value.version,
544            },
545            OpReader {
546                // Ensure concurrent is at least 1
547                concurrent: value.concurrent.max(1),
548                chunk: value.chunk,
549                gap: value.gap,
550                prefetch: value.prefetch,
551            },
552        )
553    }
554}
555
556/// Args for `stat` operation.
557#[derive(Debug, Clone, Default)]
558pub struct OpStat {
559    if_match: Option<String>,
560    if_none_match: Option<String>,
561    if_modified_since: Option<Timestamp>,
562    if_unmodified_since: Option<Timestamp>,
563    override_content_type: Option<String>,
564    override_cache_control: Option<String>,
565    override_content_disposition: Option<String>,
566    version: Option<String>,
567}
568
569impl OpStat {
570    /// Create a new `OpStat`.
571    pub fn new() -> Self {
572        Self::default()
573    }
574
575    /// Set the If-Match of the option
576    pub fn with_if_match(mut self, if_match: &str) -> Self {
577        self.if_match = Some(if_match.to_string());
578        self
579    }
580
581    /// Get If-Match from option
582    pub fn if_match(&self) -> Option<&str> {
583        self.if_match.as_deref()
584    }
585
586    /// Set the If-None-Match of the option
587    pub fn with_if_none_match(mut self, if_none_match: &str) -> Self {
588        self.if_none_match = Some(if_none_match.to_string());
589        self
590    }
591
592    /// Get If-None-Match from option
593    pub fn if_none_match(&self) -> Option<&str> {
594        self.if_none_match.as_deref()
595    }
596
597    /// Set the If-Modified-Since of the option
598    pub fn with_if_modified_since(mut self, v: Timestamp) -> Self {
599        self.if_modified_since = Some(v);
600        self
601    }
602
603    /// Get If-Modified-Since from option
604    pub fn if_modified_since(&self) -> Option<Timestamp> {
605        self.if_modified_since
606    }
607
608    /// Set the If-Unmodified-Since of the option
609    pub fn with_if_unmodified_since(mut self, v: Timestamp) -> Self {
610        self.if_unmodified_since = Some(v);
611        self
612    }
613
614    /// Get If-Unmodified-Since from option
615    pub fn if_unmodified_since(&self) -> Option<Timestamp> {
616        self.if_unmodified_since
617    }
618
619    /// Sets the content-disposition header that should be sent back by the remote read operation.
620    pub fn with_override_content_disposition(mut self, content_disposition: &str) -> Self {
621        self.override_content_disposition = Some(content_disposition.into());
622        self
623    }
624
625    /// Returns the content-disposition header that should be sent back by the remote read
626    /// operation.
627    pub fn override_content_disposition(&self) -> Option<&str> {
628        self.override_content_disposition.as_deref()
629    }
630
631    /// Sets the cache-control header that should be sent back by the remote read operation.
632    pub fn with_override_cache_control(mut self, cache_control: &str) -> Self {
633        self.override_cache_control = Some(cache_control.into());
634        self
635    }
636
637    /// Returns the cache-control header that should be sent back by the remote read operation.
638    pub fn override_cache_control(&self) -> Option<&str> {
639        self.override_cache_control.as_deref()
640    }
641
642    /// Sets the content-type header that should be sent back by the remote read operation.
643    pub fn with_override_content_type(mut self, content_type: &str) -> Self {
644        self.override_content_type = Some(content_type.into());
645        self
646    }
647
648    /// Returns the content-type header that should be sent back by the remote read operation.
649    pub fn override_content_type(&self) -> Option<&str> {
650        self.override_content_type.as_deref()
651    }
652
653    /// Set the version of the option
654    pub fn with_version(mut self, version: &str) -> Self {
655        self.version = Some(version.to_string());
656        self
657    }
658
659    /// Get version from option
660    pub fn version(&self) -> Option<&str> {
661        self.version.as_deref()
662    }
663}
664
665impl From<options::StatOptions> for OpStat {
666    fn from(value: options::StatOptions) -> Self {
667        Self {
668            if_match: value.if_match,
669            if_none_match: value.if_none_match,
670            if_modified_since: value.if_modified_since,
671            if_unmodified_since: value.if_unmodified_since,
672            override_content_type: value.override_content_type,
673            override_cache_control: value.override_cache_control,
674            override_content_disposition: value.override_content_disposition,
675            version: value.version,
676        }
677    }
678}
679
680/// Args for `write` operation.
681#[derive(Debug, Clone, Default)]
682pub struct OpWrite {
683    append: bool,
684    concurrent: usize,
685    content_type: Option<String>,
686    content_disposition: Option<String>,
687    content_encoding: Option<String>,
688    cache_control: Option<String>,
689    if_match: Option<String>,
690    if_none_match: Option<String>,
691    if_not_exists: bool,
692    user_metadata: Option<HashMap<String, String>>,
693}
694
695impl OpWrite {
696    /// Create a new `OpWrite`.
697    ///
698    /// If input path is not a file path, an error will be returned.
699    pub fn new() -> Self {
700        Self::default()
701    }
702
703    /// Get the append from op.
704    ///
705    /// The append is the flag to indicate that this write operation is an append operation.
706    pub fn append(&self) -> bool {
707        self.append
708    }
709
710    /// Set the append mode of op.
711    ///
712    /// If the append mode is set, the data will be appended to the end of the file.
713    ///
714    /// # Notes
715    ///
716    /// Service could return `Unsupported` if the underlying storage does not support append.
717    pub fn with_append(mut self, append: bool) -> Self {
718        self.append = append;
719        self
720    }
721
722    /// Get the content type from option
723    pub fn content_type(&self) -> Option<&str> {
724        self.content_type.as_deref()
725    }
726
727    /// Set the content type of option
728    pub fn with_content_type(mut self, content_type: &str) -> Self {
729        self.content_type = Some(content_type.to_string());
730        self
731    }
732
733    /// Get the content disposition from option
734    pub fn content_disposition(&self) -> Option<&str> {
735        self.content_disposition.as_deref()
736    }
737
738    /// Set the content disposition of option
739    pub fn with_content_disposition(mut self, content_disposition: &str) -> Self {
740        self.content_disposition = Some(content_disposition.to_string());
741        self
742    }
743
744    /// Get the content encoding from option
745    pub fn content_encoding(&self) -> Option<&str> {
746        self.content_encoding.as_deref()
747    }
748
749    /// Set the content encoding of option
750    pub fn with_content_encoding(mut self, content_encoding: &str) -> Self {
751        self.content_encoding = Some(content_encoding.to_string());
752        self
753    }
754
755    /// Get the cache control from option
756    pub fn cache_control(&self) -> Option<&str> {
757        self.cache_control.as_deref()
758    }
759
760    /// Set the content type of option
761    pub fn with_cache_control(mut self, cache_control: &str) -> Self {
762        self.cache_control = Some(cache_control.to_string());
763        self
764    }
765
766    /// Get the concurrent.
767    pub fn concurrent(&self) -> usize {
768        self.concurrent
769    }
770
771    /// Set the maximum concurrent write task amount.
772    pub fn with_concurrent(mut self, concurrent: usize) -> Self {
773        self.concurrent = concurrent;
774        self
775    }
776
777    /// Set the If-Match of the option
778    pub fn with_if_match(mut self, s: &str) -> Self {
779        self.if_match = Some(s.to_string());
780        self
781    }
782
783    /// Get If-Match from option
784    pub fn if_match(&self) -> Option<&str> {
785        self.if_match.as_deref()
786    }
787
788    /// Set the If-None-Match of the option
789    pub fn with_if_none_match(mut self, s: &str) -> Self {
790        self.if_none_match = Some(s.to_string());
791        self
792    }
793
794    /// Get If-None-Match from option
795    pub fn if_none_match(&self) -> Option<&str> {
796        self.if_none_match.as_deref()
797    }
798
799    /// Set the If-Not-Exist of the option
800    pub fn with_if_not_exists(mut self, b: bool) -> Self {
801        self.if_not_exists = b;
802        self
803    }
804
805    /// Get If-Not-Exist from option
806    pub fn if_not_exists(&self) -> bool {
807        self.if_not_exists
808    }
809
810    /// Set the user defined metadata of the op
811    pub fn with_user_metadata(mut self, metadata: HashMap<String, String>) -> Self {
812        self.user_metadata = Some(metadata);
813        self
814    }
815
816    /// Get the user defined metadata from the op
817    pub fn user_metadata(&self) -> Option<&HashMap<String, String>> {
818        self.user_metadata.as_ref()
819    }
820}
821
822/// Args for `writer` operation.
823#[derive(Debug, Clone, Default)]
824pub struct OpWriter {
825    chunk: Option<usize>,
826}
827
828impl OpWriter {
829    /// Create a new `OpWriter`.
830    pub fn new() -> Self {
831        Self::default()
832    }
833
834    /// Get the chunk from op.
835    ///
836    /// The chunk is used by service to decide the chunk size of the underlying writer.
837    pub fn chunk(&self) -> Option<usize> {
838        self.chunk
839    }
840
841    /// Set the chunk of op.
842    ///
843    /// If chunk is set, the data will be chunked by the underlying writer.
844    ///
845    /// ## NOTE
846    ///
847    /// Service could have their own minimum chunk size while perform write
848    /// operations like multipart uploads. So the chunk size may be larger than
849    /// the given buffer size.
850    pub fn with_chunk(mut self, chunk: usize) -> Self {
851        self.chunk = Some(chunk);
852        self
853    }
854}
855
856impl From<options::WriteOptions> for (OpWrite, OpWriter) {
857    fn from(value: options::WriteOptions) -> Self {
858        (
859            OpWrite {
860                append: value.append,
861                // Ensure concurrent is at least 1
862                concurrent: value.concurrent.max(1),
863                content_type: value.content_type,
864                content_disposition: value.content_disposition,
865                content_encoding: value.content_encoding,
866                cache_control: value.cache_control,
867                if_match: value.if_match,
868                if_none_match: value.if_none_match,
869                if_not_exists: value.if_not_exists,
870                user_metadata: value.user_metadata,
871            },
872            OpWriter { chunk: value.chunk },
873        )
874    }
875}
876
877/// Args for `copy` operation.
878#[derive(Debug, Clone, Default)]
879pub struct OpCopy {
880    if_not_exists: bool,
881}
882
883impl OpCopy {
884    /// Create a new `OpCopy`.
885    pub fn new() -> Self {
886        Self::default()
887    }
888
889    /// Set the if_not_exists flag for the operation.
890    ///
891    /// When set to true, the copy operation will only proceed if the destination
892    /// doesn't already exist.
893    pub fn with_if_not_exists(mut self, if_not_exists: bool) -> Self {
894        self.if_not_exists = if_not_exists;
895        self
896    }
897
898    /// Get if_not_exists flag.
899    pub fn if_not_exists(&self) -> bool {
900        self.if_not_exists
901    }
902}
903
904/// Args for `rename` operation.
905#[derive(Debug, Clone, Default)]
906pub struct OpRename {}
907
908impl OpRename {
909    /// Create a new `OpMove`.
910    pub fn new() -> Self {
911        Self::default()
912    }
913}
```
