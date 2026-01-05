# 

opendal/layers/

capability_check.rs

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
22use crate::layers::correctness_check::new_unsupported_error;
23use crate::raw::*;
24
25/// Add an extra capability check layer for every operation
26///
27/// Similar to `CorrectnessChecker`, Before performing any operations, this layer will first verify
28/// its arguments against the capability of the underlying service. If the arguments is not supported,
29/// an error will be returned directly.
30///
31/// Notes
32///
33/// There are two main differences between this checker with the `CorrectnessChecker`:
34/// 1. This checker provides additional checks for capabilities like write_with_content_type and
35///    list_with_versions, among others. These capabilities do not affect data integrity, even if
36///    the underlying storage services do not support them.
37///
38/// 2. OpenDAL doesn't apply this checker by default. Users can enable this layer if they want to
39///    enforce stricter requirements.
40///
41/// # examples
42///
43/// ```no_run
44/// # use opendal::layers::CapabilityCheckLayer;
45/// # use opendal::services;
46/// # use opendal::Operator;
47/// # use opendal::Result;
48/// # use opendal::Scheme;
49///
50/// # fn main() -> Result<()> {
51/// use opendal::layers::CapabilityCheckLayer;
52/// let _ = Operator::new(services::Memory::default())?
53///     .layer(CapabilityCheckLayer)
54///     .finish();
55/// Ok(())
56/// # }
57/// ```
58#[derive(Default)]
59pub struct CapabilityCheckLayer;
60
61impl<A: Access> Layer<A> for CapabilityCheckLayer {
62    type LayeredAccess = CapabilityAccessor<A>;
63
64    fn layer(&self, inner: A) -> Self::LayeredAccess {
65        let info = inner.info();
66
67        CapabilityAccessor { info, inner }
68    }
69}
70pub struct CapabilityAccessor<A: Access> {
71    info: Arc<AccessorInfo>,
72    inner: A,
73}
74
75impl<A: Access> Debug for CapabilityAccessor<A> {
76    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
77        f.debug_struct("CapabilityCheckAccessor")
78            .field("inner", &self.inner)
79            .finish_non_exhaustive()
80    }
81}
82
83impl<A: Access> LayeredAccess for CapabilityAccessor<A> {
84    type Inner = A;
85    type Reader = A::Reader;
86    type Writer = A::Writer;
87    type Lister = A::Lister;
88    type Deleter = A::Deleter;
89
90    fn inner(&self) -> &Self::Inner {
91        &self.inner
92    }
93
94    async fn read(&self, path: &str, args: OpRead) -> crate::Result<(RpRead, Self::Reader)> {
95        self.inner.read(path, args).await
96    }
97
98    async fn write(&self, path: &str, args: OpWrite) -> crate::Result<(RpWrite, Self::Writer)> {
99        let capability = self.info.full_capability();
100        if !capability.write_with_content_type && args.content_type().is_some() {
101            return Err(new_unsupported_error(
102                self.info.as_ref(),
103                Operation::Write,
104                "content_type",
105            ));
106        }
107        if !capability.write_with_cache_control && args.cache_control().is_some() {
108            return Err(new_unsupported_error(
109                self.info.as_ref(),
110                Operation::Write,
111                "cache_control",
112            ));
113        }
114        if !capability.write_with_content_disposition && args.content_disposition().is_some() {
115            return Err(new_unsupported_error(
116                self.info.as_ref(),
117                Operation::Write,
118                "content_disposition",
119            ));
120        }
121
122        self.inner.write(path, args).await
123    }
124
125    async fn delete(&self) -> crate::Result<(RpDelete, Self::Deleter)> {
126        self.inner.delete().await
127    }
128
129    async fn list(&self, path: &str, args: OpList) -> crate::Result<(RpList, Self::Lister)> {
130        let capability = self.info.full_capability();
131        if !capability.list_with_versions && args.versions() {
132            return Err(new_unsupported_error(
133                self.info.as_ref(),
134                Operation::List,
135                "version",
136            ));
137        }
138
139        self.inner.list(path, args).await
140    }
141}
142
143#[cfg(test)]
144mod tests {
145    use super::*;
146    use crate::Capability;
147    use crate::ErrorKind;
148    use crate::Operator;
149
150    #[derive(Debug)]
151    struct MockService {
152        capability: Capability,
153    }
154
155    impl Access for MockService {
156        type Reader = oio::Reader;
157        type Writer = oio::Writer;
158        type Lister = oio::Lister;
159        type Deleter = oio::Deleter;
160
161        fn info(&self) -> Arc<AccessorInfo> {
162            let info = AccessorInfo::default();
163            info.set_native_capability(self.capability);
164
165            info.into()
166        }
167
168        async fn write(&self, _: &str, _: OpWrite) -> crate::Result<(RpWrite, Self::Writer)> {
169            Ok((RpWrite::new(), Box::new(())))
170        }
171
172        async fn list(&self, _: &str, _: OpList) -> crate::Result<(RpList, Self::Lister)> {
173            Ok((RpList {}, Box::new(())))
174        }
175    }
176
177    fn new_test_operator(capability: Capability) -> Operator {
178        let srv = MockService { capability };
179
180        Operator::from_inner(Arc::new(srv)).layer(CapabilityCheckLayer)
181    }
182
183    #[tokio::test]
184    async fn test_writer_with() {
185        let op = new_test_operator(Capability {
186            write: true,
187            ..Default::default()
188        });
189        let res = op.writer_with("path").content_type("type").await;
190        assert!(res.is_err());
191
192        let res = op.writer_with("path").cache_control("cache").await;
193        assert!(res.is_err());
194
195        let res = op
196            .writer_with("path")
197            .content_disposition("disposition")
198            .await;
199        assert!(res.is_err());
200
201        let op = new_test_operator(Capability {
202            write: true,
203            write_with_content_type: true,
204            write_with_cache_control: true,
205            write_with_content_disposition: true,
206            ..Default::default()
207        });
208        let res = op.writer_with("path").content_type("type").await;
209        assert!(res.is_ok());
210
211        let res = op.writer_with("path").cache_control("cache").await;
212        assert!(res.is_ok());
213
214        let res = op
215            .writer_with("path")
216            .content_disposition("disposition")
217            .await;
218        assert!(res.is_ok());
219    }
220
221    #[tokio::test]
222    async fn test_list_with() {
223        let op = new_test_operator(Capability {
224            list: true,
225            ..Default::default()
226        });
227        let res = op.list_with("path/").versions(true).await;
228        assert!(res.is_err());
229        assert_eq!(res.unwrap_err().kind(), ErrorKind::Unsupported);
230
231        let op = new_test_operator(Capability {
232            list: true,
233            list_with_versions: true,
234            ..Default::default()
235        });
236        let res = op.lister_with("path/").versions(true).await;
237        assert!(res.is_ok())
238    }
239}
```
