# 

opendal/types/

error.rs

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
18//! Errors that returned by OpenDAL
19//!
20//! # Examples
21//!
22//! ```
23//! # use anyhow::Result;
24//! # use opendal::EntryMode;
25//! # use opendal::Operator;
26//! use opendal::ErrorKind;
27//! # async fn test(op: Operator) -> Result<()> {
28//! if let Err(e) = op.stat("test_file").await {
29//!     if e.kind() == ErrorKind::NotFound {
30//!         println!("entry not exist")
31//!     }
32//! }
33//! # Ok(())
34//! # }
35//! ```
36
37use std::backtrace::Backtrace;
38use std::backtrace::BacktraceStatus;
39use std::fmt;
40use std::fmt::Debug;
41use std::fmt::Display;
42use std::fmt::Formatter;
43use std::io;
44
45/// Result that is a wrapper of `Result<T, opendal::Error>`
46pub type Result<T, E = Error> = std::result::Result<T, E>;
47
48/// ErrorKind is all kinds of Error of opendal.
49#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
50#[non_exhaustive]
51pub enum ErrorKind {
52    /// OpenDAL don't know what happened here, and no actions other than just
53    /// returning it back. For example, s3 returns an internal service error.
54    Unexpected,
55    /// Underlying service doesn't support this operation.
56    Unsupported,
57
58    /// The config for backend is invalid.
59    ConfigInvalid,
60    /// The given path is not found.
61    NotFound,
62    /// The given path doesn't have enough permission for this operation
63    PermissionDenied,
64    /// The given path is a directory.
65    IsADirectory,
66    /// The given path is not a directory.
67    NotADirectory,
68    /// The given path already exists thus we failed to the specified operation on it.
69    AlreadyExists,
70    /// Requests that sent to this path is over the limit, please slow down.
71    RateLimited,
72    /// The given file paths are same.
73    IsSameFile,
74    /// The condition of this operation is not match.
75    ///
76    /// The `condition` itself is context based.
77    ///
78    /// For example, in S3, the `condition` can be:
79    /// 1. writing a file with If-Match header but the file's ETag is not match (will get a 412 Precondition Failed).
80    /// 2. reading a file with If-None-Match header but the file's ETag is match (will get a 304 Not Modified).
81    ///
82    /// As OpenDAL cannot handle the `condition not match` error, it will always return this error to users.
83    /// So users could to handle this error by themselves.
84    ConditionNotMatch,
85    /// The range of the content is not satisfied.
86    ///
87    /// OpenDAL returns this error to indicate that the range of the read request is not satisfied.
88    RangeNotSatisfied,
89}
90
91impl ErrorKind {
92    /// Convert self into static str.
93    pub fn into_static(self) -> &'static str {
94        self.into()
95    }
96
97    /// Capturing a backtrace can be a quite expensive runtime operation.
98    /// For some kinds of errors, backtrace is not useful and we can skip it (e.g., check if a file exists).
99    ///
100    /// See <https://github.com/apache/opendal/discussions/5569>
101    fn enable_backtrace(&self) -> bool {
102        matches!(self, ErrorKind::Unexpected)
103    }
104}
105
106impl Display for ErrorKind {
107    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
108        write!(f, "{}", self.into_static())
109    }
110}
111
112impl From<ErrorKind> for &'static str {
113    fn from(v: ErrorKind) -> &'static str {
114        match v {
115            ErrorKind::Unexpected => "Unexpected",
116            ErrorKind::Unsupported => "Unsupported",
117            ErrorKind::ConfigInvalid => "ConfigInvalid",
118            ErrorKind::NotFound => "NotFound",
119            ErrorKind::PermissionDenied => "PermissionDenied",
120            ErrorKind::IsADirectory => "IsADirectory",
121            ErrorKind::NotADirectory => "NotADirectory",
122            ErrorKind::AlreadyExists => "AlreadyExists",
123            ErrorKind::RateLimited => "RateLimited",
124            ErrorKind::IsSameFile => "IsSameFile",
125            ErrorKind::ConditionNotMatch => "ConditionNotMatch",
126            ErrorKind::RangeNotSatisfied => "RangeNotSatisfied",
127        }
128    }
129}
130
131#[derive(Clone, Copy, Debug, PartialEq, Eq)]
132enum ErrorStatus {
133    /// Permanent means without external changes, the error never changes.
134    ///
135    /// For example, underlying services returns a not found error.
136    ///
137    /// Users MUST never retry this operation.
138    Permanent,
139    /// Temporary means this error is returned for temporary.
140    ///
141    /// For example, underlying services is rate limited or unavailable for temporary.
142    ///
143    /// Users CAN retry the operation to resolve it.
144    Temporary,
145    /// Persistent means this error used to be temporary but still failed after retry.
146    ///
147    /// For example, underlying services kept returning network errors.
148    ///
149    /// Users SHOULD NOT retry this operation, since it's highly possible to error again.
150    Persistent,
151}
152
153impl Display for ErrorStatus {
154    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
155        match self {
156            ErrorStatus::Permanent => write!(f, "permanent"),
157            ErrorStatus::Temporary => write!(f, "temporary"),
158            ErrorStatus::Persistent => write!(f, "persistent"),
159        }
160    }
161}
162
163/// Error is the error struct returned by all opendal functions.
164///
165/// ## Display
166///
167/// Error can be displayed in two ways:
168///
169/// - Via `Display`: like `err.to_string()` or `format!("{err}")`
170///
171/// Error will be printed in a single line:
172///
173/// ```shell
174/// Unexpected, context: { path: /path/to/file, called: send_async } => something wrong happened, source: networking error"
175/// ```
176///
177/// - Via `Debug`: like `format!("{err:?}")`
178///
179/// Error will be printed in multi lines with more details and backtraces (if captured):
180///
181/// ```shell
182/// Unexpected => something wrong happened
183///
184/// Context:
185///    path: /path/to/file
186///    called: send_async
187///
188/// Source: networking error
189///
190/// Backtrace:
191///    0: opendal::error::Error::new
192///              at ./src/error.rs:197:24
193///    1: opendal::error::tests::generate_error
194///              at ./src/error.rs:241:9
195///    2: opendal::error::tests::test_error_debug_with_backtrace::{{closure}}
196///              at ./src/error.rs:305:41
197///    ...
198/// ```
199///
200/// - For conventional struct-style Debug representation, like `format!("{err:#?}")`:
201///
202/// ```shell
203/// Error {
204///     kind: Unexpected,
205///     message: "something wrong happened",
206///     status: Permanent,
207///     operation: "Read",
208///     context: [
209///         (
210///             "path",
211///             "/path/to/file",
212///         ),
213///         (
214///             "called",
215///             "send_async",
216///         ),
217///     ],
218///     source: Some(
219///         "networking error",
220///     ),
221/// }
222/// ```
223pub struct Error {
224    kind: ErrorKind,
225    message: String,
226
227    status: ErrorStatus,
228    operation: &'static str,
229    context: Vec<(&'static str, String)>,
230
231    source: Option<anyhow::Error>,
232    backtrace: Option<Box<Backtrace>>,
233}
234
235impl Display for Error {
236    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
237        write!(f, "{} ({}) at {}", self.kind, self.status, self.operation)?;
238
239        if !self.context.is_empty() {
240            write!(f, ", context: {{ ")?;
241            write!(
242                f,
243                "{}",
244                self.context
245                    .iter()
246                    .map(|(k, v)| format!("{k}: {v}"))
247                    .collect::<Vec<_>>()
248                    .join(", ")
249            )?;
250            write!(f, " }}")?;
251        }
252
253        if !self.message.is_empty() {
254            write!(f, " => {}", self.message)?;
255        }
256
257        if let Some(source) = &self.source {
258            write!(f, ", source: {source}")?;
259        }
260
261        Ok(())
262    }
263}
264
265impl Debug for Error {
266    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
267        // If alternate has been specified, we will print like Debug.
268        if f.alternate() {
269            let mut de = f.debug_struct("Error");
270            de.field("kind", &self.kind);
271            de.field("message", &self.message);
272            de.field("status", &self.status);
273            de.field("operation", &self.operation);
274            de.field("context", &self.context);
275            de.field("source", &self.source);
276            return de.finish();
277        }
278
279        write!(f, "{} ({}) at {}", self.kind, self.status, self.operation)?;
280        if !self.message.is_empty() {
281            write!(f, " => {}", self.message)?;
282        }
283        writeln!(f)?;
284
285        if !self.context.is_empty() {
286            writeln!(f)?;
287            writeln!(f, "Context:")?;
288            for (k, v) in self.context.iter() {
289                writeln!(f, "   {k}: {v}")?;
290            }
291        }
292        if let Some(source) = &self.source {
293            writeln!(f)?;
294            writeln!(f, "Source:")?;
295            writeln!(f, "   {source:#}")?;
296        }
297
298        if let Some(backtrace) = &self.backtrace {
299            writeln!(f)?;
300            writeln!(f, "Backtrace:")?;
301            writeln!(f, "{backtrace}")?;
302        }
303
304        Ok(())
305    }
306}
307
308impl std::error::Error for Error {
309    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
310        self.source.as_ref().map(|v| v.as_ref())
311    }
312}
313
314impl Error {
315    /// Create a new Error with error kind and message.
316    pub fn new(kind: ErrorKind, message: impl Into<String>) -> Self {
317        Self {
318            kind,
319            message: message.into(),
320
321            status: ErrorStatus::Permanent,
322            operation: "",
323            context: Vec::default(),
324            source: None,
325
326            backtrace: kind
327                .enable_backtrace()
328                // `Backtrace::capture()` will check if backtrace has
329                // been enabled internally. It's zero cost if backtrace
330                // is disabled.
331                .then(Backtrace::capture)
332                // We only keep captured backtrace to avoid an extra box.
333                .filter(|bt| bt.status() == BacktraceStatus::Captured)
334                .map(Box::new),
335        }
336    }
337
338    /// Update error's operation.
339    ///
340    /// # Notes
341    ///
342    /// If the error already carries an operation, we will push a new context
343    /// `(called, operation)`.
344    pub fn with_operation(mut self, operation: impl Into<&'static str>) -> Self {
345        if !self.operation.is_empty() {
346            self.context.push(("called", self.operation.to_string()));
347        }
348
349        self.operation = operation.into();
350        self
351    }
352
353    /// Add more context in error.
354    pub fn with_context(mut self, key: &'static str, value: impl ToString) -> Self {
355        self.context.push((key, value.to_string()));
356        self
357    }
358
359    /// Set source for error.
360    ///
361    /// # Notes
362    ///
363    /// If the source has been set, we will raise a panic here.
364    pub fn set_source(mut self, src: impl Into<anyhow::Error>) -> Self {
365        debug_assert!(self.source.is_none(), "the source error has been set");
366
367        self.source = Some(src.into());
368        self
369    }
370
371    /// Operate on error with map.
372    pub fn map<F>(self, f: F) -> Self
373    where
374        F: FnOnce(Self) -> Self,
375    {
376        f(self)
377    }
378
379    /// Set permanent status for error.
380    ///
381    /// By set permanent, we indicate the retry must be stopped.
382    pub fn set_permanent(mut self) -> Self {
383        self.status = ErrorStatus::Permanent;
384        self
385    }
386
387    /// Set temporary status for error.
388    ///
389    /// By set temporary, we indicate this error is retryable.
390    pub fn set_temporary(mut self) -> Self {
391        self.status = ErrorStatus::Temporary;
392        self
393    }
394
395    /// Set persistent status for error.
396    ///
397    /// By setting persistent, we indicate the retry should be stopped.
398    pub fn set_persistent(mut self) -> Self {
399        self.status = ErrorStatus::Persistent;
400        self
401    }
402
403    /// Set permanent status for error by given permanent.
404    ///
405    /// By set permanent, we indicate the retry must be stopped.
406    pub fn with_permanent(mut self, permanent: bool) -> Self {
407        if permanent {
408            self.status = ErrorStatus::Permanent;
409        }
410        self
411    }
412
413    /// Set temporary status for error by given temporary.
414    ///
415    /// By set temporary, we indicate this error is retryable.
416    pub fn with_temporary(mut self, temporary: bool) -> Self {
417        if temporary {
418            self.status = ErrorStatus::Temporary;
419        }
420        self
421    }
422
423    /// Set persistent status for error by given persistent.
424    ///
425    /// By set persistent, we indicate the retry should be stopped.
426    pub fn with_persistent(mut self, persistent: bool) -> Self {
427        if persistent {
428            self.status = ErrorStatus::Persistent;
429        }
430        self
431    }
432
433    /// Return error's kind.
434    pub fn kind(&self) -> ErrorKind {
435        self.kind
436    }
437
438    /// Check if this error is permanent.
439    pub fn is_permanent(&self) -> bool {
440        self.status == ErrorStatus::Permanent
441    }
442
443    /// Check if this error is temporary.
444    pub fn is_temporary(&self) -> bool {
445        self.status == ErrorStatus::Temporary
446    }
447
448    /// Check if this error is persistent.
449    pub fn is_persistent(&self) -> bool {
450        self.status == ErrorStatus::Persistent
451    }
452
453    /// Return error's backtrace.
454    ///
455    /// Note: the standard way of exposing backtrace is the unstable feature [`error_generic_member_access`](https://github.com/rust-lang/rust/issues/99301).
456    /// We don't provide it as it requires nightly rust.
457    ///
458    /// If you just want to print error with backtrace, use `Debug`, like `format!("{err:?}")`.
459    ///
460    /// If you use nightly rust, and want to access `opendal::Error`'s backtrace in the standard way, you can
461    /// implement a newtype like this:
462    ///
463    /// ```ignore
464    /// // assume you already have `#![feature(error_generic_member_access)]` on the top of your crate
465    ///
466    /// #[derive(::std::fmt::Debug)]
467    /// pub struct OpendalError(opendal::Error);
468    ///
469    /// impl std::fmt::Display for OpendalError {
470    ///     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
471    ///         self.0.fmt(f)
472    ///     }
473    /// }
474    ///
475    /// impl std::error::Error for OpendalError {
476    ///     fn provide<'a>(&'a self, request: &mut std::error::Request<'a>) {
477    ///         if let Some(bt) = self.0.backtrace() {
478    ///             request.provide_ref::<std::backtrace::Backtrace>(bt);
479    ///         }
480    ///     }
481    ///
482    ///     fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
483    ///         self.0.source()
484    ///     }
485    /// }
486    /// ```
487    ///
488    /// Additionally, you can add a clippy lint to prevent usage of the original `opendal::Error` type.
489    /// ```toml
490    /// disallowed-types = [
491    ///     { path = "opendal::Error", reason = "Please use `my_crate::OpendalError` instead." },
492    /// ]
493    /// ```
494    pub fn backtrace(&self) -> Option<&Backtrace> {
495        self.backtrace.as_ref().map(|bt| bt.as_ref())
496    }
497}
498
499impl From<Error> for io::Error {
500    fn from(err: Error) -> Self {
501        let kind = match err.kind() {
502            ErrorKind::NotFound => io::ErrorKind::NotFound,
503            ErrorKind::PermissionDenied => io::ErrorKind::PermissionDenied,
504            _ => io::ErrorKind::Other,
505        };
506
507        io::Error::new(kind, err)
508    }
509}
510
511#[cfg(test)]
512mod tests {
513    use std::mem::size_of;
514    use std::sync::LazyLock;
515
516    use anyhow::anyhow;
517    use pretty_assertions::assert_eq;
518
519    use super::*;
520
521    static TEST_ERROR: LazyLock<Error> = LazyLock::new(|| Error {
522        kind: ErrorKind::Unexpected,
523        message: "something wrong happened".to_string(),
524        status: ErrorStatus::Permanent,
525        operation: "Read",
526        context: vec![
527            ("path", "/path/to/file".to_string()),
528            ("called", "send_async".to_string()),
529        ],
530        source: Some(anyhow!("networking error")),
531        backtrace: None,
532    });
533
534    /// This is not a real test case.
535    ///
536    /// We assert our public structs here to make sure we don't introduce
537    /// unexpected struct/enum size change.
538    #[cfg(target_pointer_width = "64")]
539    #[test]
540    fn assert_size() {
541        assert_eq!(88, size_of::<Error>());
542    }
543
544    #[test]
545    fn test_error_display() {
546        let s = format!("{}", LazyLock::force(&TEST_ERROR));
547        assert_eq!(
548            s,
549            r#"Unexpected (permanent) at Read, context: { path: /path/to/file, called: send_async } => something wrong happened, source: networking error"#
550        );
551        println!("{:#?}", LazyLock::force(&TEST_ERROR));
552    }
553
554    #[test]
555    fn test_error_debug() {
556        let s = format!("{:?}", LazyLock::force(&TEST_ERROR));
557        assert_eq!(
558            s,
559            r#"Unexpected (permanent) at Read => something wrong happened
560
561Context:
562   path: /path/to/file
563   called: send_async
564
565Source:
566   networking error
567"#
568        )
569    }
570}
```
