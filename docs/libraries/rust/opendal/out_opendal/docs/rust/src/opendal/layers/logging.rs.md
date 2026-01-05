# 

opendal/layers/

logging.rs

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
19use std::fmt::Display;
20use std::sync::Arc;
21
22use log::Level;
23use log::log;
24
25use crate::raw::*;
26use crate::*;
27
28/// Add [log](https://docs.rs/log/) for every operation.
29///
30/// # Logging
31///
32/// - OpenDAL will log in structural way.
33/// - Every operation will start with a `started` log entry.
34/// - Every operation will finish with the following status:
35///   - `succeeded`: the operation is successful, but might have more to take.
36///   - `finished`: the whole operation is finished.
37///   - `failed`: the operation returns an unexpected error.
38/// - The default log level while expected error happened is `Warn`.
39/// - The default log level while unexpected failure happened is `Error`.
40///
41/// # Examples
42///
43/// ```no_run
44/// # use opendal::layers::LoggingLayer;
45/// # use opendal::services;
46/// # use opendal::Operator;
47/// # use opendal::Result;
48/// # use opendal::Scheme;
49///
50/// # fn main() -> Result<()> {
51/// let _ = Operator::new(services::Memory::default())?
52///     .layer(LoggingLayer::default())
53///     .finish();
54/// Ok(())
55/// # }
56/// ```
57///
58/// # Output
59///
60/// OpenDAL is using [`log`](https://docs.rs/log/latest/log/) for logging internally.
61///
62/// To enable logging output, please set `RUST_LOG`:
63///
64/// ```shell
65/// RUST_LOG=debug ./app
66/// ```
67///
68/// To config logging output, please refer to [Configure Logging](https://rust-lang-nursery.github.io/rust-cookbook/development_tools/debugging/config_log.html):
69///
70/// ```shell
71/// RUST_LOG="info,opendal::services=debug" ./app
72/// ```
73///
74/// # Logging Interceptor
75///
76/// You can implement your own logging interceptor to customize the logging behavior.
77///
78/// ```no_run
79/// # use opendal::layers::LoggingInterceptor;
80/// # use opendal::layers::LoggingLayer;
81/// # use opendal::raw;
82/// # use opendal::services;
83/// # use opendal::Error;
84/// # use opendal::Operator;
85/// # use opendal::Result;
86/// # use opendal::Scheme;
87///
88/// #[derive(Debug, Clone)]
89/// struct MyLoggingInterceptor;
90///
91/// impl LoggingInterceptor for MyLoggingInterceptor {
92///     fn log(
93///         &self,
94///         info: &raw::AccessorInfo,
95///         operation: raw::Operation,
96///         context: &[(&str, &str)],
97///         message: &str,
98///         err: Option<&Error>,
99///     ) {
100///         // log something
101///     }
102/// }
103///
104/// # fn main() -> Result<()> {
105/// let _ = Operator::new(services::Memory::default())?
106///     .layer(LoggingLayer::new(MyLoggingInterceptor))
107///     .finish();
108/// Ok(())
109/// # }
110/// ```
111#[derive(Debug)]
112pub struct LoggingLayer<I = DefaultLoggingInterceptor> {
113    logger: I,
114}
115
116impl Default for LoggingLayer {
117    fn default() -> Self {
118        Self {
119            logger: DefaultLoggingInterceptor,
120        }
121    }
122}
123
124impl LoggingLayer {
125    /// Create the layer with specific logging interceptor.
126    pub fn new<I: LoggingInterceptor>(logger: I) -> LoggingLayer<I> {
127        LoggingLayer { logger }
128    }
129}
130
131impl<A: Access, I: LoggingInterceptor> Layer<A> for LoggingLayer<I> {
132    type LayeredAccess = LoggingAccessor<A, I>;
133
134    fn layer(&self, inner: A) -> Self::LayeredAccess {
135        let info = inner.info();
136        LoggingAccessor {
137            inner,
138
139            info,
140            logger: self.logger.clone(),
141        }
142    }
143}
144
145/// LoggingInterceptor is used to intercept the log.
146pub trait LoggingInterceptor: Debug + Clone + Send + Sync + Unpin + 'static {
147    /// Everytime there is a log, this function will be called.
148    ///
149    /// # Inputs
150    ///
151    /// - info: The service's access info.
152    /// - operation: The operation to log.
153    /// - context: Additional context of the log like path, etc.
154    /// - message: The log message.
155    /// - err: The error to log.
156    ///
157    /// # Note
158    ///
159    /// Users should avoid calling resource-intensive operations such as I/O or network
160    /// functions here, especially anything that takes longer than 10ms. Otherwise, Opendal
161    /// could perform unexpectedly slow.
162    fn log(
163        &self,
164        info: &AccessorInfo,
165        operation: Operation,
166        context: &[(&str, &str)],
167        message: &str,
168        err: Option<&Error>,
169    );
170}
171
172/// The DefaultLoggingInterceptor will log the message by the standard logging macro.
173#[derive(Debug, Copy, Clone, Default)]
174pub struct DefaultLoggingInterceptor;
175
176impl LoggingInterceptor for DefaultLoggingInterceptor {
177    #[inline]
178    fn log(
179        &self,
180        info: &AccessorInfo,
181        operation: Operation,
182        context: &[(&str, &str)],
183        message: &str,
184        err: Option<&Error>,
185    ) {
186        if let Some(err) = err {
187            // Print error if it's unexpected, otherwise in warn.
188            let lvl = if err.kind() == ErrorKind::Unexpected {
189                Level::Error
190            } else {
191                Level::Warn
192            };
193
194            log!(
195                target: LOGGING_TARGET,
196                lvl,
197                "service={} name={}{}: {operation} {message} {}",
198                info.scheme(),
199                info.name(),
200                LoggingContext(context),
201                // Print error message with debug output while unexpected happened.
202                //
203                // It's super sad that we can't bind `format_args!()` here.
204                // See: https://github.com/rust-lang/rust/issues/92698
205                if err.kind() != ErrorKind::Unexpected {
206                   format!("{err}")
207                } else {
208                   format!("{err:?}")
209                }
210            );
211        }
212
213        log!(
214            target: LOGGING_TARGET,
215            Level::Debug,
216            "service={} name={}{}: {operation} {message}",
217            info.scheme(),
218            info.name(),
219            LoggingContext(context),
220        );
221    }
222}
223
224struct LoggingContext<'a>(&'a [(&'a str, &'a str)]);
225
226impl Display for LoggingContext<'_> {
227    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
228        for (k, v) in self.0.iter() {
229            write!(f, " {k}={v}")?;
230        }
231        Ok(())
232    }
233}
234
235#[derive(Clone, Debug)]
236pub struct LoggingAccessor<A: Access, I: LoggingInterceptor> {
237    inner: A,
238
239    info: Arc<AccessorInfo>,
240    logger: I,
241}
242
243static LOGGING_TARGET: &str = "opendal::services";
244
245impl<A: Access, I: LoggingInterceptor> LayeredAccess for LoggingAccessor<A, I> {
246    type Inner = A;
247    type Reader = LoggingReader<A::Reader, I>;
248    type Writer = LoggingWriter<A::Writer, I>;
249    type Lister = LoggingLister<A::Lister, I>;
250    type Deleter = LoggingDeleter<A::Deleter, I>;
251
252    fn inner(&self) -> &Self::Inner {
253        &self.inner
254    }
255
256    fn info(&self) -> Arc<AccessorInfo> {
257        self.info.clone()
258    }
259
260    async fn create_dir(&self, path: &str, args: OpCreateDir) -> Result<RpCreateDir> {
261        self.logger.log(
262            &self.info,
263            Operation::CreateDir,
264            &[("path", path)],
265            "started",
266            None,
267        );
268
269        self.inner
270            .create_dir(path, args)
271            .await
272            .inspect(|_| {
273                self.logger.log(
274                    &self.info,
275                    Operation::CreateDir,
276                    &[("path", path)],
277                    "finished",
278                    None,
279                );
280            })
281            .inspect_err(|err| {
282                self.logger.log(
283                    &self.info,
284                    Operation::CreateDir,
285                    &[("path", path)],
286                    "failed",
287                    Some(err),
288                );
289            })
290    }
291
292    async fn read(&self, path: &str, args: OpRead) -> Result<(RpRead, Self::Reader)> {
293        self.logger.log(
294            &self.info,
295            Operation::Read,
296            &[("path", path)],
297            "started",
298            None,
299        );
300
301        self.inner
302            .read(path, args)
303            .await
304            .map(|(rp, r)| {
305                self.logger.log(
306                    &self.info,
307                    Operation::Read,
308                    &[("path", path)],
309                    "created reader",
310                    None,
311                );
312                (
313                    rp,
314                    LoggingReader::new(self.info.clone(), self.logger.clone(), path, r),
315                )
316            })
317            .inspect_err(|err| {
318                self.logger.log(
319                    &self.info,
320                    Operation::Read,
321                    &[("path", path)],
322                    "failed",
323                    Some(err),
324                );
325            })
326    }
327
328    async fn write(&self, path: &str, args: OpWrite) -> Result<(RpWrite, Self::Writer)> {
329        self.logger.log(
330            &self.info,
331            Operation::Write,
332            &[("path", path)],
333            "started",
334            None,
335        );
336
337        self.inner
338            .write(path, args)
339            .await
340            .map(|(rp, w)| {
341                self.logger.log(
342                    &self.info,
343                    Operation::Write,
344                    &[("path", path)],
345                    "created writer",
346                    None,
347                );
348                let w = LoggingWriter::new(self.info.clone(), self.logger.clone(), path, w);
349                (rp, w)
350            })
351            .inspect_err(|err| {
352                self.logger.log(
353                    &self.info,
354                    Operation::Write,
355                    &[("path", path)],
356                    "failed",
357                    Some(err),
358                );
359            })
360    }
361
362    async fn copy(&self, from: &str, to: &str, args: OpCopy) -> Result<RpCopy> {
363        self.logger.log(
364            &self.info,
365            Operation::Copy,
366            &[("from", from), ("to", to)],
367            "started",
368            None,
369        );
370
371        self.inner
372            .copy(from, to, args)
373            .await
374            .inspect(|_| {
375                self.logger.log(
376                    &self.info,
377                    Operation::Copy,
378                    &[("from", from), ("to", to)],
379                    "finished",
380                    None,
381                );
382            })
383            .inspect_err(|err| {
384                self.logger.log(
385                    &self.info,
386                    Operation::Copy,
387                    &[("from", from), ("to", to)],
388                    "failed",
389                    Some(err),
390                );
391            })
392    }
393
394    async fn rename(&self, from: &str, to: &str, args: OpRename) -> Result<RpRename> {
395        self.logger.log(
396            &self.info,
397            Operation::Rename,
398            &[("from", from), ("to", to)],
399            "started",
400            None,
401        );
402
403        self.inner
404            .rename(from, to, args)
405            .await
406            .inspect(|_| {
407                self.logger.log(
408                    &self.info,
409                    Operation::Rename,
410                    &[("from", from), ("to", to)],
411                    "finished",
412                    None,
413                );
414            })
415            .inspect_err(|err| {
416                self.logger.log(
417                    &self.info,
418                    Operation::Rename,
419                    &[("from", from), ("to", to)],
420                    "failed",
421                    Some(err),
422                );
423            })
424    }
425
426    async fn stat(&self, path: &str, args: OpStat) -> Result<RpStat> {
427        self.logger.log(
428            &self.info,
429            Operation::Stat,
430            &[("path", path)],
431            "started",
432            None,
433        );
434
435        self.inner
436            .stat(path, args)
437            .await
438            .inspect(|_| {
439                self.logger.log(
440                    &self.info,
441                    Operation::Stat,
442                    &[("path", path)],
443                    "finished",
444                    None,
445                );
446            })
447            .inspect_err(|err| {
448                self.logger.log(
449                    &self.info,
450                    Operation::Stat,
451                    &[("path", path)],
452                    "failed",
453                    Some(err),
454                );
455            })
456    }
457
458    async fn delete(&self) -> Result<(RpDelete, Self::Deleter)> {
459        self.logger
460            .log(&self.info, Operation::Delete, &[], "started", None);
461
462        self.inner
463            .delete()
464            .await
465            .map(|(rp, d)| {
466                self.logger
467                    .log(&self.info, Operation::Delete, &[], "finished", None);
468                let d = LoggingDeleter::new(self.info.clone(), self.logger.clone(), d);
469                (rp, d)
470            })
471            .inspect_err(|err| {
472                self.logger
473                    .log(&self.info, Operation::Delete, &[], "failed", Some(err));
474            })
475    }
476
477    async fn list(&self, path: &str, args: OpList) -> Result<(RpList, Self::Lister)> {
478        self.logger.log(
479            &self.info,
480            Operation::List,
481            &[("path", path)],
482            "started",
483            None,
484        );
485
486        self.inner
487            .list(path, args)
488            .await
489            .map(|(rp, v)| {
490                self.logger.log(
491                    &self.info,
492                    Operation::List,
493                    &[("path", path)],
494                    "created lister",
495                    None,
496                );
497                let streamer = LoggingLister::new(self.info.clone(), self.logger.clone(), path, v);
498                (rp, streamer)
499            })
500            .inspect_err(|err| {
501                self.logger.log(
502                    &self.info,
503                    Operation::List,
504                    &[("path", path)],
505                    "failed",
506                    Some(err),
507                );
508            })
509    }
510
511    async fn presign(&self, path: &str, args: OpPresign) -> Result<RpPresign> {
512        self.logger.log(
513            &self.info,
514            Operation::Presign,
515            &[("path", path)],
516            "started",
517            None,
518        );
519
520        self.inner
521            .presign(path, args)
522            .await
523            .inspect(|_| {
524                self.logger.log(
525                    &self.info,
526                    Operation::Presign,
527                    &[("path", path)],
528                    "finished",
529                    None,
530                );
531            })
532            .inspect_err(|err| {
533                self.logger.log(
534                    &self.info,
535                    Operation::Presign,
536                    &[("path", path)],
537                    "failed",
538                    Some(err),
539                );
540            })
541    }
542}
543
544pub struct LoggingReader<R, I: LoggingInterceptor> {
545    info: Arc<AccessorInfo>,
546    logger: I,
547    path: String,
548
549    read: u64,
550    inner: R,
551}
552
553impl<R, I: LoggingInterceptor> LoggingReader<R, I> {
554    fn new(info: Arc<AccessorInfo>, logger: I, path: &str, reader: R) -> Self {
555        Self {
556            info,
557            logger,
558            path: path.to_string(),
559
560            read: 0,
561            inner: reader,
562        }
563    }
564}
565
566impl<R: oio::Read, I: LoggingInterceptor> oio::Read for LoggingReader<R, I> {
567    async fn read(&mut self) -> Result<Buffer> {
568        match self.inner.read().await {
569            Ok(bs) if bs.is_empty() => {
570                self.logger.log(
571                    &self.info,
572                    Operation::Read,
573                    &[
574                        ("path", &self.path),
575                        ("read", &self.read.to_string()),
576                        ("size", &bs.len().to_string()),
577                    ],
578                    "finished",
579                    None,
580                );
581                Ok(bs)
582            }
583            Ok(bs) => {
584                self.read += bs.len() as u64;
585                Ok(bs)
586            }
587            Err(err) => {
588                self.logger.log(
589                    &self.info,
590                    Operation::Read,
591                    &[("path", &self.path), ("read", &self.read.to_string())],
592                    "failed",
593                    Some(&err),
594                );
595                Err(err)
596            }
597        }
598    }
599}
600
601pub struct LoggingWriter<W, I> {
602    info: Arc<AccessorInfo>,
603    logger: I,
604    path: String,
605
606    written: u64,
607    inner: W,
608}
609
610impl<W, I> LoggingWriter<W, I> {
611    fn new(info: Arc<AccessorInfo>, logger: I, path: &str, writer: W) -> Self {
612        Self {
613            info,
614            logger,
615            path: path.to_string(),
616
617            written: 0,
618            inner: writer,
619        }
620    }
621}
622
623impl<W: oio::Write, I: LoggingInterceptor> oio::Write for LoggingWriter<W, I> {
624    async fn write(&mut self, bs: Buffer) -> Result<()> {
625        let size = bs.len();
626
627        match self.inner.write(bs).await {
628            Ok(_) => {
629                self.written += size as u64;
630                Ok(())
631            }
632            Err(err) => {
633                self.logger.log(
634                    &self.info,
635                    Operation::Write,
636                    &[
637                        ("path", &self.path),
638                        ("written", &self.written.to_string()),
639                        ("size", &size.to_string()),
640                    ],
641                    "failed",
642                    Some(&err),
643                );
644                Err(err)
645            }
646        }
647    }
648
649    async fn abort(&mut self) -> Result<()> {
650        match self.inner.abort().await {
651            Ok(_) => {
652                self.logger.log(
653                    &self.info,
654                    Operation::Write,
655                    &[("path", &self.path), ("written", &self.written.to_string())],
656                    "abort succeeded",
657                    None,
658                );
659                Ok(())
660            }
661            Err(err) => {
662                self.logger.log(
663                    &self.info,
664                    Operation::Write,
665                    &[("path", &self.path), ("written", &self.written.to_string())],
666                    "abort failed",
667                    Some(&err),
668                );
669                Err(err)
670            }
671        }
672    }
673
674    async fn close(&mut self) -> Result<Metadata> {
675        match self.inner.close().await {
676            Ok(meta) => {
677                self.logger.log(
678                    &self.info,
679                    Operation::Write,
680                    &[("path", &self.path), ("written", &self.written.to_string())],
681                    "close succeeded",
682                    None,
683                );
684                Ok(meta)
685            }
686            Err(err) => {
687                self.logger.log(
688                    &self.info,
689                    Operation::Write,
690                    &[("path", &self.path), ("written", &self.written.to_string())],
691                    "close failed",
692                    Some(&err),
693                );
694                Err(err)
695            }
696        }
697    }
698}
699
700pub struct LoggingLister<P, I: LoggingInterceptor> {
701    info: Arc<AccessorInfo>,
702    logger: I,
703    path: String,
704
705    listed: usize,
706    inner: P,
707}
708
709impl<P, I: LoggingInterceptor> LoggingLister<P, I> {
710    fn new(info: Arc<AccessorInfo>, logger: I, path: &str, inner: P) -> Self {
711        Self {
712            info,
713            logger,
714            path: path.to_string(),
715
716            listed: 0,
717            inner,
718        }
719    }
720}
721
722impl<P: oio::List, I: LoggingInterceptor> oio::List for LoggingLister<P, I> {
723    async fn next(&mut self) -> Result<Option<oio::Entry>> {
724        let res = self.inner.next().await;
725
726        match &res {
727            Ok(Some(_)) => {
728                self.listed += 1;
729            }
730            Ok(None) => {
731                self.logger.log(
732                    &self.info,
733                    Operation::List,
734                    &[("path", &self.path), ("listed", &self.listed.to_string())],
735                    "finished",
736                    None,
737                );
738            }
739            Err(err) => {
740                self.logger.log(
741                    &self.info,
742                    Operation::List,
743                    &[("path", &self.path), ("listed", &self.listed.to_string())],
744                    "failed",
745                    Some(err),
746                );
747            }
748        };
749
750        res
751    }
752}
753
754pub struct LoggingDeleter<D, I: LoggingInterceptor> {
755    info: Arc<AccessorInfo>,
756    logger: I,
757
758    queued: usize,
759    deleted: usize,
760    inner: D,
761}
762
763impl<D, I: LoggingInterceptor> LoggingDeleter<D, I> {
764    fn new(info: Arc<AccessorInfo>, logger: I, inner: D) -> Self {
765        Self {
766            info,
767            logger,
768
769            queued: 0,
770            deleted: 0,
771            inner,
772        }
773    }
774}
775
776impl<D: oio::Delete, I: LoggingInterceptor> oio::Delete for LoggingDeleter<D, I> {
777    fn delete(&mut self, path: &str, args: OpDelete) -> Result<()> {
778        let version = args
779            .version()
780            .map(|v| v.to_string())
781            .unwrap_or_else(|| "<latest>".to_string());
782
783        let res = self.inner.delete(path, args);
784
785        match &res {
786            Ok(_) => {
787                self.queued += 1;
788            }
789            Err(err) => {
790                self.logger.log(
791                    &self.info,
792                    Operation::Delete,
793                    &[
794                        ("path", path),
795                        ("version", &version),
796                        ("queued", &self.queued.to_string()),
797                        ("deleted", &self.deleted.to_string()),
798                    ],
799                    "failed",
800                    Some(err),
801                );
802            }
803        };
804
805        res
806    }
807
808    async fn flush(&mut self) -> Result<usize> {
809        let res = self.inner.flush().await;
810
811        match &res {
812            Ok(flushed) => {
813                self.queued -= flushed;
814                self.deleted += flushed;
815                self.logger.log(
816                    &self.info,
817                    Operation::Delete,
818                    &[
819                        ("queued", &self.queued.to_string()),
820                        ("deleted", &self.deleted.to_string()),
821                    ],
822                    "succeeded",
823                    None,
824                );
825            }
826            Err(err) => {
827                self.logger.log(
828                    &self.info,
829                    Operation::Delete,
830                    &[
831                        ("queued", &self.queued.to_string()),
832                        ("deleted", &self.deleted.to_string()),
833                    ],
834                    "failed",
835                    Some(err),
836                );
837            }
838        };
839
840        res
841    }
842}
```
