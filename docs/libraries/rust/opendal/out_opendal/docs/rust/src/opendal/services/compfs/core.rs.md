# 

opendal/services/compfs/

core.rs

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
18use std::future::Future;
19use std::path::PathBuf;
20use std::sync::Arc;
21
22use compio::buf::IoBuf;
23use compio::dispatcher::Dispatcher;
24
25use crate::raw::*;
26use crate::*;
27
28unsafe impl IoBuf for Buffer {
29    fn as_buf_ptr(&self) -> *const u8 {
30        self.current().as_ptr()
31    }
32
33    fn buf_len(&self) -> usize {
34        self.current().len()
35    }
36
37    fn buf_capacity(&self) -> usize {
38        // `Bytes` doesn't expose uninitialized capacity, so treat it as the same as `len`
39        self.current().len()
40    }
41}
42
43#[derive(Debug)]
44pub(super) struct CompfsCore {
45    pub info: Arc<AccessorInfo>,
46
47    pub root: PathBuf,
48    pub dispatcher: Dispatcher,
49    pub buf_pool: oio::PooledBuf,
50}
51
52impl CompfsCore {
53    pub fn prepare_path(&self, path: &str) -> PathBuf {
54        self.root.join(path.trim_end_matches('/'))
55    }
56
57    pub async fn exec<Fn, Fut, R>(&self, f: Fn) -> crate::Result<R>
58    where
59        Fn: FnOnce() -> Fut + Send + 'static,
60        Fut: Future<Output = std::io::Result<R>> + 'static,
61        R: Send + 'static,
62    {
63        self.dispatcher
64            .dispatch(f)
65            .map_err(|_| Error::new(ErrorKind::Unexpected, "compio spawn io task failed"))?
66            .await
67            .map_err(|_| Error::new(ErrorKind::Unexpected, "compio task cancelled"))?
68            .map_err(new_std_io_error)
69    }
70
71    pub async fn exec_blocking<Fn, R>(&self, f: Fn) -> Result<R>
72    where
73        Fn: FnOnce() -> R + Send + 'static,
74        R: Send + 'static,
75    {
76        self.dispatcher
77            .dispatch_blocking(f)
78            .map_err(|_| Error::new(ErrorKind::Unexpected, "compio spawn blocking task failed"))?
79            .await
80            .map_err(|_| Error::new(ErrorKind::Unexpected, "compio task cancelled"))
81    }
82}
83
84// TODO: impl IoVectoredBuf for Buffer
85// impl IoVectoredBuf for Buffer {
86//     fn as_dyn_bufs(&self) -> impl Iterator<Item = &dyn IoBuf> {}
87//
88//     fn owned_iter(self) -> Result<OwnedIter<impl OwnedIterator<Inner = Self>>, Self> {
89//         Ok(OwnedIter::new(BufferIter {
90//             current: self.current(),
91//             buf: self,
92//         }))
93//     }
94// }
95
96// #[derive(Debug, Clone)]
97// struct BufferIter {
98//     buf: Buffer,
99//     current: Bytes,
100// }
101
102// impl IntoInner for BufferIter {
103//     type Inner = Buffer;
104//
105//     fn into_inner(self) -> Self::Inner {
106//         self.buf
107//     }
108// }
109
110// impl OwnedIterator for BufferIter {
111//     fn next(mut self) -> Result<Self, Self::Inner> {
112//         let Some(current) = self.buf.next() else {
113//             return Err(self.buf);
114//         };
115//         self.current = current;
116//         Ok(self)
117//     }
118//
119//     fn current(&self) -> &dyn IoBuf {
120//         &self.current
121//     }
122// }
123
124#[cfg(test)]
125mod tests {
126    use bytes::Buf;
127    use bytes::Bytes;
128    use rand::Rng;
129    use rand::thread_rng;
130
131    use super::*;
132
133    fn setup_buffer() -> (Buffer, usize, Bytes) {
134        let mut rng = thread_rng();
135
136        let bs = (0..100)
137            .map(|_| {
138                let len = rng.gen_range(1..100);
139                let mut buf = vec![0; len];
140                rng.fill(&mut buf[..]);
141                Bytes::from(buf)
142            })
143            .collect::<Vec<_>>();
144
145        let total_size = bs.iter().map(|b| b.len()).sum::<usize>();
146        let total_content = bs.iter().flatten().copied().collect::<Bytes>();
147        let buf = Buffer::from(bs);
148
149        (buf, total_size, total_content)
150    }
151
152    #[test]
153    fn test_io_buf() {
154        let (buf, _len, _bytes) = setup_buffer();
155        let slice = IoBuf::as_slice(&buf);
156
157        assert_eq!(slice, buf.current().chunk())
158    }
159}
```
