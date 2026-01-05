# 

opendal/types/

buffer.rs

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
19use std::convert::Infallible;
20use std::fmt::Debug;
21use std::fmt::Formatter;
22use std::io::BufRead;
23use std::io::IoSlice;
24use std::io::Read;
25use std::io::Seek;
26use std::io::SeekFrom;
27use std::io::{self};
28use std::iter::FusedIterator;
29use std::mem;
30use std::ops::Bound;
31use std::ops::RangeBounds;
32use std::pin::Pin;
33use std::sync::Arc;
34use std::task::Context;
35use std::task::Poll;
36
37use bytes::Buf;
38use bytes::BufMut;
39use bytes::Bytes;
40use bytes::BytesMut;
41use futures::Stream;
42
43use crate::*;
44
45/// Buffer is a wrapper of contiguous `Bytes` and non-contiguous `[Bytes]`.
46///
47/// We designed buffer to allow underlying storage to return non-contiguous bytes. For example,
48/// http based storage like s3 could generate non-contiguous bytes by stream.
49///
50/// ## Features
51///
52/// - [`Buffer`] can be used as [`Buf`], [`Iterator`], [`Stream`] directly.
53/// - [`Buffer`] is cheap to clone like [`Bytes`], only update reference count, no allocation.
54/// - [`Buffer`] is vectorized write friendly, you can convert it to [`IoSlice`] for vectored write.
55///
56/// ## Examples
57///
58/// ### As `Buf`
59///
60/// `Buffer` implements `Buf` trait:
61///
62/// ```rust
63/// use bytes::Buf;
64/// use opendal::Buffer;
65/// use serde_json;
66///
67/// fn test(mut buf: Buffer) -> Vec<String> {
68///     serde_json::from_reader(buf.reader()).unwrap()
69/// }
70/// ```
71///
72/// ### As Bytes `Iterator`
73///
74/// `Buffer` implements `Iterator<Item=Bytes>` trait:
75///
76/// ```rust
77/// use bytes::Bytes;
78/// use opendal::Buffer;
79///
80/// fn test(mut buf: Buffer) -> Vec<Bytes> {
81///     buf.into_iter().collect()
82/// }
83/// ```
84///
85/// ### As Bytes `Stream`
86///
87/// `Buffer` implements `Stream<Item=Result<Bytes, Infallible>>` trait:
88///
89/// ```rust
90/// use bytes::Bytes;
91/// use futures::TryStreamExt;
92/// use opendal::Buffer;
93///
94/// async fn test(mut buf: Buffer) -> Vec<Bytes> {
95///     buf.into_iter().try_collect().await.unwrap()
96/// }
97/// ```
98///
99/// ### As one contiguous Bytes
100///
101/// `Buffer` can make contiguous by transform into `Bytes` or `Vec<u8>`.
102/// Please keep in mind that this operation involves new allocation and bytes copy, and we can't
103/// reuse the same memory region anymore.
104///
105/// ```rust
106/// use bytes::Bytes;
107/// use opendal::Buffer;
108///
109/// fn test_to_vec(buf: Buffer) -> Vec<u8> {
110///     buf.to_vec()
111/// }
112///
113/// fn test_to_bytes(buf: Buffer) -> Bytes {
114///     buf.to_bytes()
115/// }
116/// ```
117#[derive(Clone)]
118pub struct Buffer(Inner);
119
120#[derive(Clone)]
121enum Inner {
122    Contiguous(Bytes),
123    NonContiguous {
124        parts: Arc<[Bytes]>,
125        size: usize,
126        idx: usize,
127        offset: usize,
128    },
129}
130
131impl Debug for Buffer {
132    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
133        let mut b = f.debug_struct("Buffer");
134
135        match &self.0 {
136            Inner::Contiguous(bs) => {
137                b.field("type", &"contiguous");
138                b.field("size", &bs.len());
139            }
140            Inner::NonContiguous {
141                parts,
142                size,
143                idx,
144                offset,
145            } => {
146                b.field("type", &"non_contiguous");
147                b.field("parts", &parts);
148                b.field("size", &size);
149                b.field("idx", &idx);
150                b.field("offset", &offset);
151            }
152        }
153        b.finish_non_exhaustive()
154    }
155}
156
157impl Default for Buffer {
158    fn default() -> Self {
159        Self::new()
160    }
161}
162
163impl Buffer {
164    /// Create a new empty buffer.
165    ///
166    /// This operation is const and no allocation will be performed.
167    #[inline]
168    pub const fn new() -> Self {
169        Self(Inner::Contiguous(Bytes::new()))
170    }
171
172    /// Get the length of the buffer.
173    #[inline]
174    pub fn len(&self) -> usize {
175        match &self.0 {
176            Inner::Contiguous(b) => b.remaining(),
177            Inner::NonContiguous { size, .. } => *size,
178        }
179    }
180
181    /// Check if buffer is empty.
182    #[inline]
183    pub fn is_empty(&self) -> bool {
184        self.len() == 0
185    }
186
187    /// Number of [`Bytes`] in [`Buffer`].
188    ///
189    /// For contiguous buffer, it's always 1. For non-contiguous buffer, it's number of bytes
190    /// available for use.
191    pub fn count(&self) -> usize {
192        match &self.0 {
193            Inner::Contiguous(_) => 1,
194            Inner::NonContiguous {
195                parts,
196                idx,
197                size,
198                offset,
199            } => {
200                parts
201                    .iter()
202                    .skip(*idx)
203                    .fold((0, size + offset), |(count, size), bytes| {
204                        if size == 0 {
205                            (count, 0)
206                        } else {
207                            (count + 1, size.saturating_sub(bytes.len()))
208                        }
209                    })
210                    .0
211            }
212        }
213    }
214
215    /// Get current [`Bytes`].
216    pub fn current(&self) -> Bytes {
217        match &self.0 {
218            Inner::Contiguous(inner) => inner.clone(),
219            Inner::NonContiguous {
220                parts,
221                idx,
222                offset,
223                size,
224            } => {
225                let chunk = &parts[*idx];
226                let n = (chunk.len() - *offset).min(*size);
227                chunk.slice(*offset..*offset + n)
228            }
229        }
230    }
231
232    /// Shortens the buffer, keeping the first `len` bytes and dropping the rest.
233    ///
234    /// If `len` is greater than the bufferâs current length, this has no effect.
235    #[inline]
236    pub fn truncate(&mut self, len: usize) {
237        match &mut self.0 {
238            Inner::Contiguous(bs) => bs.truncate(len),
239            Inner::NonContiguous { size, .. } => {
240                *size = (*size).min(len);
241            }
242        }
243    }
244
245    /// Returns a slice of self for the provided range.
246    ///
247    /// This will increment the reference count for the underlying memory and return a new Buffer handle set to the slice.
248    ///
249    /// This operation is O(1).
250    pub fn slice(&self, range: impl RangeBounds<usize>) -> Self {
251        let len = self.len();
252
253        let begin = match range.start_bound() {
254            Bound::Included(&n) => n,
255            Bound::Excluded(&n) => n.checked_add(1).expect("out of range"),
256            Bound::Unbounded => 0,
257        };
258
259        let end = match range.end_bound() {
260            Bound::Included(&n) => n.checked_add(1).expect("out of range"),
261            Bound::Excluded(&n) => n,
262            Bound::Unbounded => len,
263        };
264
265        assert!(
266            begin <= end,
267            "range start must not be greater than end: {begin:?} <= {end:?}",
268        );
269        assert!(end <= len, "range end out of bounds: {end:?} <= {len:?}",);
270
271        if end == begin {
272            return Buffer::new();
273        }
274
275        let mut ret = self.clone();
276        ret.truncate(end);
277        ret.advance(begin);
278        ret
279    }
280
281    /// Combine all bytes together into one single [`Bytes`].
282    ///
283    /// This operation is zero copy if the underlying bytes are contiguous.
284    /// Otherwise, it will copy all bytes into one single [`Bytes`].
285    /// Please use API from [`Buf`], [`Iterator`] or [`Stream`] whenever possible.
286    #[inline]
287    pub fn to_bytes(&self) -> Bytes {
288        match &self.0 {
289            Inner::Contiguous(bytes) => bytes.clone(),
290            Inner::NonContiguous {
291                parts,
292                size,
293                idx: _,
294                offset,
295            } => {
296                if parts.len() == 1 {
297                    parts[0].slice(*offset..(*offset + *size))
298                } else {
299                    let mut ret = BytesMut::with_capacity(self.len());
300                    ret.put(self.clone());
301                    ret.freeze()
302                }
303            }
304        }
305    }
306
307    /// Combine all bytes together into one single [`Vec<u8>`].
308    ///
309    /// This operation is not zero copy, it will copy all bytes into one single [`Vec<u8>`].
310    /// Please use API from [`Buf`], [`Iterator`] or [`Stream`] whenever possible.
311    #[inline]
312    pub fn to_vec(&self) -> Vec<u8> {
313        let mut ret = Vec::with_capacity(self.len());
314        ret.put(self.clone());
315        ret
316    }
317
318    /// Convert buffer into a slice of [`IoSlice`] for vectored write.
319    #[inline]
320    pub fn to_io_slice(&self) -> Vec<IoSlice<'_>> {
321        match &self.0 {
322            Inner::Contiguous(bs) => vec![IoSlice::new(bs.chunk())],
323            Inner::NonContiguous {
324                parts, idx, offset, ..
325            } => {
326                let mut ret = Vec::with_capacity(parts.len() - *idx);
327                let mut new_offset = *offset;
328                for part in parts.iter().skip(*idx) {
329                    ret.push(IoSlice::new(&part[new_offset..]));
330                    new_offset = 0;
331                }
332                ret
333            }
334        }
335    }
336
337    /// Split the buffer into an iterator of chunks, each with at most `chunk_size` bytes.
338    ///
339    /// The chunks share the same underlying storage with the original buffer. The last chunk
340    /// will be shorter if `self.len()` is not a multiple of `chunk_size`.
341    ///
342    /// # Panics
343    ///
344    /// Panics if `chunk_size` is zero.
345    pub fn chunks(&self, chunk_size: usize) -> BufferChunks {
346        assert!(chunk_size != 0, "chunk size must be greater than 0");
347
348        BufferChunks {
349            buffer: self.clone(),
350            chunk_size,
351            position: 0,
352            len: self.len(),
353        }
354    }
355}
356
357impl From<Vec<u8>> for Buffer {
358    #[inline]
359    fn from(bs: Vec<u8>) -> Self {
360        Self(Inner::Contiguous(bs.into()))
361    }
362}
363
364impl From<Bytes> for Buffer {
365    #[inline]
366    fn from(bs: Bytes) -> Self {
367        Self(Inner::Contiguous(bs))
368    }
369}
370
371impl From<String> for Buffer {
372    #[inline]
373    fn from(s: String) -> Self {
374        Self(Inner::Contiguous(Bytes::from(s)))
375    }
376}
377
378impl From<&'static [u8]> for Buffer {
379    #[inline]
380    fn from(s: &'static [u8]) -> Self {
381        Self(Inner::Contiguous(Bytes::from_static(s)))
382    }
383}
384
385impl From<&'static str> for Buffer {
386    #[inline]
387    fn from(s: &'static str) -> Self {
388        Self(Inner::Contiguous(Bytes::from_static(s.as_bytes())))
389    }
390}
391
392impl FromIterator<u8> for Buffer {
393    #[inline]
394    fn from_iter<T: IntoIterator<Item = u8>>(iter: T) -> Self {
395        Self(Inner::Contiguous(Bytes::from_iter(iter)))
396    }
397}
398
399impl From<VecDeque<Bytes>> for Buffer {
400    #[inline]
401    fn from(bs: VecDeque<Bytes>) -> Self {
402        let size = bs.iter().map(Bytes::len).sum();
403        Self(Inner::NonContiguous {
404            parts: Vec::from(bs).into(),
405            size,
406            idx: 0,
407            offset: 0,
408        })
409    }
410}
411
412impl From<Vec<Bytes>> for Buffer {
413    #[inline]
414    fn from(bs: Vec<Bytes>) -> Self {
415        let size = bs.iter().map(Bytes::len).sum();
416        Self(Inner::NonContiguous {
417            parts: bs.into(),
418            size,
419            idx: 0,
420            offset: 0,
421        })
422    }
423}
424
425impl From<Arc<[Bytes]>> for Buffer {
426    #[inline]
427    fn from(bs: Arc<[Bytes]>) -> Self {
428        let size = bs.iter().map(Bytes::len).sum();
429        Self(Inner::NonContiguous {
430            parts: bs,
431            size,
432            idx: 0,
433            offset: 0,
434        })
435    }
436}
437
438impl FromIterator<Bytes> for Buffer {
439    #[inline]
440    fn from_iter<T: IntoIterator<Item = Bytes>>(iter: T) -> Self {
441        let mut size = 0;
442        let bs = iter.into_iter().inspect(|v| size += v.len());
443        // This operation only needs one allocation from iterator to `Arc<[Bytes]>` instead
444        // of iterator -> `Vec<Bytes>` -> `Arc<[Bytes]>`.
445        let parts = Arc::from_iter(bs);
446        Self(Inner::NonContiguous {
447            parts,
448            size,
449            idx: 0,
450            offset: 0,
451        })
452    }
453}
454
455impl Buf for Buffer {
456    #[inline]
457    fn remaining(&self) -> usize {
458        self.len()
459    }
460
461    #[inline]
462    fn chunk(&self) -> &[u8] {
463        match &self.0 {
464            Inner::Contiguous(b) => b.chunk(),
465            Inner::NonContiguous {
466                parts,
467                size,
468                idx,
469                offset,
470            } => {
471                if *size == 0 {
472                    return &[];
473                }
474
475                let chunk = &parts[*idx];
476                let n = (chunk.len() - *offset).min(*size);
477                &parts[*idx][*offset..*offset + n]
478            }
479        }
480    }
481
482    #[inline]
483    fn chunks_vectored<'a>(&'a self, dst: &mut [IoSlice<'a>]) -> usize {
484        match &self.0 {
485            Inner::Contiguous(b) => {
486                if dst.is_empty() {
487                    return 0;
488                }
489
490                dst[0] = IoSlice::new(b.chunk());
491                1
492            }
493            Inner::NonContiguous {
494                parts, idx, offset, ..
495            } => {
496                if dst.is_empty() {
497                    return 0;
498                }
499
500                let mut new_offset = *offset;
501                parts
502                    .iter()
503                    .skip(*idx)
504                    .zip(dst.iter_mut())
505                    .map(|(part, dst)| {
506                        *dst = IoSlice::new(&part[new_offset..]);
507                        new_offset = 0;
508                    })
509                    .count()
510            }
511        }
512    }
513
514    #[inline]
515    fn advance(&mut self, cnt: usize) {
516        match &mut self.0 {
517            Inner::Contiguous(b) => b.advance(cnt),
518            Inner::NonContiguous {
519                parts,
520                size,
521                idx,
522                offset,
523            } => {
524                assert!(
525                    cnt <= *size,
526                    "cannot advance past {cnt} bytes, only {size} bytes left"
527                );
528
529                let mut new_idx = *idx;
530                let mut new_offset = *offset;
531                let mut remaining_cnt = cnt;
532                while remaining_cnt > 0 {
533                    let part_len = parts[new_idx].len();
534                    let remaining_in_part = part_len - new_offset;
535
536                    if remaining_cnt < remaining_in_part {
537                        new_offset += remaining_cnt;
538                        break;
539                    }
540
541                    remaining_cnt -= remaining_in_part;
542                    new_idx += 1;
543                    new_offset = 0;
544                }
545
546                *idx = new_idx;
547                *offset = new_offset;
548                *size -= cnt;
549            }
550        }
551    }
552}
553
554impl Iterator for Buffer {
555    type Item = Bytes;
556
557    fn next(&mut self) -> Option<Self::Item> {
558        match &mut self.0 {
559            Inner::Contiguous(bs) => {
560                if bs.is_empty() {
561                    None
562                } else {
563                    Some(mem::take(bs))
564                }
565            }
566            Inner::NonContiguous {
567                parts,
568                size,
569                idx,
570                offset,
571            } => {
572                if *size == 0 {
573                    return None;
574                }
575
576                let chunk = &parts[*idx];
577                let n = (chunk.len() - *offset).min(*size);
578                let buf = chunk.slice(*offset..*offset + n);
579                *size -= n;
580                *offset += n;
581
582                if *offset == chunk.len() {
583                    *idx += 1;
584                    *offset = 0;
585                }
586
587                Some(buf)
588            }
589        }
590    }
591
592    fn size_hint(&self) -> (usize, Option<usize>) {
593        match &self.0 {
594            Inner::Contiguous(bs) => {
595                if bs.is_empty() {
596                    (0, Some(0))
597                } else {
598                    (1, Some(1))
599                }
600            }
601            Inner::NonContiguous { parts, idx, .. } => {
602                let remaining = parts.len().saturating_sub(*idx);
603                (remaining, Some(remaining))
604            }
605        }
606    }
607}
608
609impl Stream for Buffer {
610    type Item = Result<Bytes, Infallible>;
611
612    fn poll_next(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Option<Self::Item>> {
613        Poll::Ready(self.get_mut().next().map(Ok))
614    }
615
616    fn size_hint(&self) -> (usize, Option<usize>) {
617        Iterator::size_hint(self)
618    }
619}
620
621impl Read for Buffer {
622    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
623        let chunk = self.chunk();
624        let len = chunk.len().min(buf.len());
625        buf[..len].copy_from_slice(&chunk[..len]);
626        self.advance(len);
627        Ok(len)
628    }
629}
630
631impl Seek for Buffer {
632    fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
633        let len = self.len() as u64;
634        let new_pos = match pos {
635            SeekFrom::Start(offset) => offset,
636            SeekFrom::End(offset) => {
637                if offset < 0 {
638                    len.checked_sub(offset.unsigned_abs())
639                        .ok_or(io::Error::new(
640                            io::ErrorKind::InvalidInput,
641                            "invalid seek to a negative position",
642                        ))?
643                } else {
644                    len.checked_add(offset as u64).ok_or(io::Error::new(
645                        io::ErrorKind::InvalidInput,
646                        "seek out of bounds",
647                    ))?
648                }
649            }
650            SeekFrom::Current(offset) => {
651                let current_pos = (len - self.remaining() as u64) as i64;
652                let new_pos = current_pos.checked_add(offset).ok_or(io::Error::new(
653                    io::ErrorKind::InvalidInput,
654                    "seek out of bounds",
655                ))?;
656                if new_pos < 0 {
657                    return Err(io::Error::new(
658                        io::ErrorKind::InvalidInput,
659                        "invalid seek to a negative position",
660                    ));
661                }
662                new_pos as u64
663            }
664        };
665
666        if new_pos > len {
667            return Err(io::Error::new(
668                io::ErrorKind::InvalidInput,
669                "seek out of bounds",
670            ));
671        }
672
673        self.advance((new_pos - (len - self.remaining() as u64)) as usize);
674        Ok(new_pos)
675    }
676}
677
678impl BufRead for Buffer {
679    fn fill_buf(&mut self) -> io::Result<&[u8]> {
680        let chunk = match &self.0 {
681            Inner::Contiguous(b) => b.chunk(),
682            Inner::NonContiguous {
683                parts,
684                size,
685                idx,
686                offset,
687            } => {
688                if *size == 0 {
689                    return Ok(&[]);
690                }
691
692                let chunk = &parts[*idx];
693                let n = (chunk.len() - *offset).min(*size);
694                &parts[*idx][*offset..*offset + n]
695            }
696        };
697        Ok(chunk)
698    }
699
700    fn consume(&mut self, amt: usize) {
701        self.advance(amt);
702    }
703}
704
705/// Iterator that yields [`Buffer`] chunks of at most a configured length.
706pub struct BufferChunks {
707    buffer: Buffer,
708    chunk_size: usize,
709    position: usize,
710    len: usize,
711}
712
713impl Iterator for BufferChunks {
714    type Item = Buffer;
715
716    fn next(&mut self) -> Option<Self::Item> {
717        if self.position >= self.len {
718            return None;
719        }
720
721        let end = (self.position + self.chunk_size).min(self.len);
722        let chunk = self.buffer.slice(self.position..end);
723        self.position = end;
724        Some(chunk)
725    }
726
727    fn size_hint(&self) -> (usize, Option<usize>) {
728        let remaining = self.len.saturating_sub(self.position);
729        let chunks = remaining.div_ceil(self.chunk_size);
730        (chunks, Some(chunks))
731    }
732}
733
734impl ExactSizeIterator for BufferChunks {
735    fn len(&self) -> usize {
736        self.size_hint().0
737    }
738}
739
740impl FusedIterator for BufferChunks {}
741
742#[cfg(test)]
743mod tests {
744    use std::io::BufRead;
745    use std::io::Read;
746    use std::io::Seek;
747    use std::io::SeekFrom;
748
749    use pretty_assertions::assert_eq;
750    use rand::prelude::*;
751
752    use super::*;
753
754    const EMPTY_SLICE: &[u8] = &[];
755
756    #[test]
757    fn test_contiguous_buffer() {
758        let mut buf = Buffer::new();
759
760        assert_eq!(buf.remaining(), 0);
761        assert_eq!(buf.chunk(), EMPTY_SLICE);
762        assert_eq!(buf.next(), None);
763    }
764
765    #[test]
766    fn test_empty_non_contiguous_buffer() {
767        let mut buf = Buffer::from(vec![Bytes::new()]);
768
769        assert_eq!(buf.remaining(), 0);
770        assert_eq!(buf.chunk(), EMPTY_SLICE);
771        assert_eq!(buf.next(), None);
772    }
773
774    #[test]
775    fn test_non_contiguous_buffer_with_empty_chunks() {
776        let mut buf = Buffer::from(vec![Bytes::from("a")]);
777
778        assert_eq!(buf.remaining(), 1);
779        assert_eq!(buf.chunk(), b"a");
780
781        buf.advance(1);
782
783        assert_eq!(buf.remaining(), 0);
784        assert_eq!(buf.chunk(), EMPTY_SLICE);
785    }
786
787    #[test]
788    fn test_non_contiguous_buffer_with_next() {
789        let mut buf = Buffer::from(vec![Bytes::from("a")]);
790
791        assert_eq!(buf.remaining(), 1);
792        assert_eq!(buf.chunk(), b"a");
793
794        let bs = buf.next();
795
796        assert_eq!(bs, Some(Bytes::from("a")));
797        assert_eq!(buf.remaining(), 0);
798        assert_eq!(buf.chunk(), EMPTY_SLICE);
799    }
800
801    #[test]
802    fn test_buffer_advance() {
803        let mut buf = Buffer::from(vec![Bytes::from("a"), Bytes::from("b"), Bytes::from("c")]);
804
805        assert_eq!(buf.remaining(), 3);
806        assert_eq!(buf.chunk(), b"a");
807
808        buf.advance(1);
809
810        assert_eq!(buf.remaining(), 2);
811        assert_eq!(buf.chunk(), b"b");
812
813        buf.advance(1);
814
815        assert_eq!(buf.remaining(), 1);
816        assert_eq!(buf.chunk(), b"c");
817
818        buf.advance(1);
819
820        assert_eq!(buf.remaining(), 0);
821        assert_eq!(buf.chunk(), EMPTY_SLICE);
822
823        buf.advance(0);
824
825        assert_eq!(buf.remaining(), 0);
826        assert_eq!(buf.chunk(), EMPTY_SLICE);
827    }
828
829    #[test]
830    fn test_buffer_truncate() {
831        let mut buf = Buffer::from(vec![Bytes::from("a"), Bytes::from("b"), Bytes::from("c")]);
832
833        assert_eq!(buf.remaining(), 3);
834        assert_eq!(buf.chunk(), b"a");
835
836        buf.truncate(100);
837
838        assert_eq!(buf.remaining(), 3);
839        assert_eq!(buf.chunk(), b"a");
840
841        buf.truncate(2);
842
843        assert_eq!(buf.remaining(), 2);
844        assert_eq!(buf.chunk(), b"a");
845
846        buf.truncate(0);
847
848        assert_eq!(buf.remaining(), 0);
849        assert_eq!(buf.chunk(), EMPTY_SLICE);
850    }
851
852    #[test]
853    fn test_buffer_chunks_contiguous() {
854        let buf = Buffer::from(Bytes::from("abcdefg"));
855
856        let chunks = buf
857            .chunks(3)
858            .map(|chunk| chunk.to_bytes())
859            .collect::<Vec<Bytes>>();
860
861        assert_eq!(
862            chunks,
863            vec![Bytes::from("abc"), Bytes::from("def"), Bytes::from("g")]
864        );
865
866        assert_eq!(Buffer::new().chunks(4).count(), 0);
867    }
868
869    #[test]
870    fn test_buffer_chunks_non_contiguous() {
871        let buf = Buffer::from(vec![
872            Bytes::from("ab"),
873            Bytes::from("c"),
874            Bytes::from("def"),
875        ]);
876
877        let chunks = buf
878            .chunks(2)
879            .map(|chunk| chunk.to_bytes())
880            .collect::<Vec<Bytes>>();
881
882        assert_eq!(
883            chunks,
884            vec![Bytes::from("ab"), Bytes::from("cd"), Bytes::from("ef"),]
885        );
886    }
887
888    #[test]
889    #[should_panic(expected = "chunk size must be greater than 0")]
890    fn test_buffer_chunks_zero_panics() {
891        let buf = Buffer::from(Bytes::from("abc"));
892        let _ = buf.chunks(0);
893    }
894
895    /// This setup will return
896    ///
897    /// - A buffer
898    /// - Total size of this buffer.
899    /// - Total content of this buffer.
900    fn setup_buffer() -> (Buffer, usize, Bytes) {
901        let mut rng = thread_rng();
902
903        let bs = (0..100)
904            .map(|_| {
905                let len = rng.gen_range(1..100);
906                let mut buf = vec![0; len];
907                rng.fill(&mut buf[..]);
908                Bytes::from(buf)
909            })
910            .collect::<Vec<_>>();
911
912        let total_size = bs.iter().map(|b| b.len()).sum::<usize>();
913        let total_content = bs.iter().flatten().copied().collect::<Bytes>();
914        let buf = Buffer::from(bs);
915
916        (buf, total_size, total_content)
917    }
918
919    #[test]
920    fn fuzz_buffer_advance() {
921        let mut rng = thread_rng();
922
923        let (mut buf, total_size, total_content) = setup_buffer();
924        assert_eq!(buf.remaining(), total_size);
925        assert_eq!(buf.to_bytes(), total_content);
926
927        let mut cur = 0;
928        // Loop at most 10000 times.
929        let mut times = 10000;
930        while !buf.is_empty() && times > 0 {
931            times -= 1;
932
933            let cnt = rng.gen_range(0..total_size - cur);
934            cur += cnt;
935            buf.advance(cnt);
936
937            assert_eq!(buf.remaining(), total_size - cur);
938            assert_eq!(buf.to_bytes(), total_content.slice(cur..));
939        }
940    }
941
942    #[test]
943    fn fuzz_buffer_iter() {
944        let mut rng = thread_rng();
945
946        let (mut buf, total_size, total_content) = setup_buffer();
947        assert_eq!(buf.remaining(), total_size);
948        assert_eq!(buf.to_bytes(), total_content);
949
950        let mut cur = 0;
951        while buf.is_empty() {
952            let cnt = rng.gen_range(0..total_size - cur);
953            cur += cnt;
954            buf.advance(cnt);
955
956            // Before next
957            assert_eq!(buf.remaining(), total_size - cur);
958            assert_eq!(buf.to_bytes(), total_content.slice(cur..));
959
960            if let Some(bs) = buf.next() {
961                assert_eq!(bs, total_content.slice(cur..cur + bs.len()));
962                cur += bs.len();
963            }
964
965            // After next
966            assert_eq!(buf.remaining(), total_size - cur);
967            assert_eq!(buf.to_bytes(), total_content.slice(cur..));
968        }
969    }
970
971    #[test]
972    fn fuzz_buffer_truncate() {
973        let mut rng = thread_rng();
974
975        let (mut buf, total_size, total_content) = setup_buffer();
976        assert_eq!(buf.remaining(), total_size);
977        assert_eq!(buf.to_bytes(), total_content);
978
979        let mut cur = 0;
980        while buf.is_empty() {
981            let cnt = rng.gen_range(0..total_size - cur);
982            cur += cnt;
983            buf.advance(cnt);
984
985            // Before truncate
986            assert_eq!(buf.remaining(), total_size - cur);
987            assert_eq!(buf.to_bytes(), total_content.slice(cur..));
988
989            let truncate_size = rng.gen_range(0..total_size - cur);
990            buf.truncate(truncate_size);
991
992            // After truncate
993            assert_eq!(buf.remaining(), truncate_size);
994            assert_eq!(
995                buf.to_bytes(),
996                total_content.slice(cur..cur + truncate_size)
997            );
998
999            // Try next after truncate
1000            if let Some(bs) = buf.next() {
1001                assert_eq!(bs, total_content.slice(cur..cur + bs.len()));
1002                cur += bs.len();
1003            }
1004
1005            // After next
1006            assert_eq!(buf.remaining(), total_size - cur);
1007            assert_eq!(buf.to_bytes(), total_content.slice(cur..));
1008        }
1009    }
1010
1011    #[test]
1012    fn test_read_trait() {
1013        let mut buffer = Buffer::from(vec![Bytes::from("Hello"), Bytes::from("World")]);
1014        let mut output = vec![0; 5];
1015        let size = buffer.read(&mut output).unwrap();
1016        assert_eq!(size, 5);
1017        assert_eq!(&output, b"Hello");
1018    }
1019
1020    #[test]
1021    fn test_seek_trait() {
1022        let mut buffer = Buffer::from(vec![Bytes::from("Hello"), Bytes::from("World")]);
1023        buffer.seek(SeekFrom::Start(5)).unwrap();
1024        let mut output = vec![0; 5];
1025        buffer.read_exact(&mut output).unwrap();
1026        assert_eq!(&output, b"World");
1027    }
1028
1029    #[test]
1030    fn test_bufread_trait() {
1031        let mut buffer = Buffer::from(vec![Bytes::from("Hello"), Bytes::from("World")]);
1032        let mut output = String::new();
1033        buffer.read_to_string(&mut output).unwrap();
1034        assert_eq!(output, "HelloWorld");
1035
1036        let mut buffer = Buffer::from(vec![Bytes::from("Hello"), Bytes::from("World")]);
1037        let buf = buffer.fill_buf().unwrap();
1038        assert_eq!(buf, b"Hello");
1039        buffer.consume(5);
1040        let buf = buffer.fill_buf().unwrap();
1041        assert_eq!(buf, b"World");
1042    }
1043
1044    #[test]
1045    fn test_read_partial() {
1046        let mut buffer = Buffer::from(vec![Bytes::from("Partial"), Bytes::from("Read")]);
1047        let mut output = vec![0; 4];
1048        let size = buffer.read(&mut output).unwrap();
1049        assert_eq!(size, 4);
1050        assert_eq!(&output, b"Part");
1051
1052        let size = buffer.read(&mut output).unwrap();
1053        assert_eq!(size, 3);
1054        assert_eq!(&output[..3], b"ial");
1055    }
1056
1057    #[test]
1058    fn test_seek_and_read() {
1059        let mut buffer = Buffer::from(vec![Bytes::from("SeekAndRead")]);
1060        buffer.seek(SeekFrom::Start(4)).unwrap();
1061        let mut output = vec![0; 3];
1062        buffer.read_exact(&mut output).unwrap();
1063        assert_eq!(&output, b"And");
1064    }
1065
1066    #[test]
1067    fn test_bufread_consume() {
1068        let mut buffer = Buffer::from(vec![Bytes::from("ConsumeTest")]);
1069        let buf = buffer.fill_buf().unwrap();
1070        assert_eq!(buf, b"ConsumeTest");
1071        buffer.consume(7);
1072        let buf = buffer.fill_buf().unwrap();
1073        assert_eq!(buf, b"Test");
1074    }
1075
1076    #[test]
1077    fn test_empty_buffer() {
1078        let mut buffer = Buffer::new();
1079        let mut output = vec![0; 5];
1080        let size = buffer.read(&mut output).unwrap();
1081        assert_eq!(size, 0);
1082        assert_eq!(&output, &[0; 5]);
1083    }
1084
1085    #[test]
1086    fn test_seek_out_of_bounds() {
1087        let mut buffer = Buffer::from(vec![Bytes::from("OutOfBounds")]);
1088        let result = buffer.seek(SeekFrom::Start(100));
1089        assert!(result.is_err());
1090    }
1091}
```
