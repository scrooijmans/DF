# 

opendal/docs/internals/

accessor.rs

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
18//! The internal implementation details of [`Access`].
19//!
20//! [`Access`] is the core trait of OpenDAL's raw API. We operate
21//! underlying storage services via APIs provided by [`Access`].
22//!
23//! # Introduction
24//!
25//! [`Access`] can be split in the following parts:
26//!
27//! ```ignore
28//! //                  <----------Trait Bound-------------->
29//! pub trait Access: Send + Sync + Debug + Unpin + 'static {
30//!     type Reader: oio::Read;                    // --+
31//!     type Writer: oio::Write;                   //   +--> Associated Type
32//!     type Lister: oio::List;                    //   +
33//!     type Deleter: oio::Delete;                 // --+
34//!
35//!     // APIs
36//!     fn info(&self) -> Arc<AccessorInfo>;
37//!     fn create_dir(
38//!         &self,
39//!         path: &str,
40//!         args: OpCreateDir,
41//!     ) -> impl core::future::Future<Output = Result<RpCreateDir>> + MaybeSend;
42//! }
43//! ```
44//!
45//! Let's go deep into [`Access`] line by line.
46//!
47//! ## Trait Bound
48//!
49//! First we will read the declare of [`Access`] trait:
50//!
51//! ```ignore
52//! pub trait Access: Send + Sync + Debug + Unpin + 'static {}
53//! ```
54//!
55//! There are many trait boundings here. For now, [`Access`] requires the following bound:
56//!
57//! - [`Send`]: Allow user to send between threads without extra wrapper.
58//! - [`Sync`]: Allow user to sync between threads without extra lock.
59//! - [`Debug`][std::fmt::Debug]: Allow users to print underlying debug information of accessor.
60//! - [`Unpin`]: Make sure `Access` can be safely moved after being pinned, so users don't need to `Pin<Box<A>>`.
61//! - `'static`: Make sure `Access` is not a short-time reference, allow users to use `Access` in closures and futures without playing with lifetime.
62//!
63//! Implementer of `Access` should take care of the following things:
64//!
65//! - Implement `Debug` for backend, but don't leak credentials.
66//! - Make sure the backend is `Send` and `Sync`, wrap the internal struct with `Arc<Mutex<T>>` if necessary.
67//!
68//! ## Associated Type
69//!
70//! The first block of [`Access`] trait is our associated types. We
71//! require implementers to specify the type to be returned, thus avoiding
72//! the additional overhead of dynamic dispatch.
73//!
74//! [`Access`] has four associated types so far:
75//!
76//! - `Reader`: reader returned by `read` operation.
77//! - `Writer`: writer returned by `write` operation.
78//! - `Lister`: lister returned by `list` operation.
79//! - `Deleter`: deleter returned by `delete` operation.
80//!
81//! Implementer of `Access` should take care the following things:
82//!
83//! - OpenDAL will erase those type at the final stage of Operator building. Please don't return dynamic trait object like `oio::Reader`.
84//! - Use `()` as type if the operation is not supported.
85//!
86//! ## API Style
87//!
88//! Every API of [`Access`] follows the same style:
89//!
90//! - All APIs have a unique [`Operation`] and [`Capability`]
91//! - All APIs are orthogonal and do not overlap with each other
92//! - Most APIs accept `path` and `OpXxx`, and returns `RpXxx`.
93//! - Most APIs have `async` and `blocking` variants, they share the same semantics but may have different underlying implementations.
94//!
95//! [`Access`] can declare their capabilities via [`AccessorInfo`]'s `set_native_capability`:
96//!
97//! ```ignore
98//! impl Access for MyBackend {
99//!     fn info(&self) -> Arc<AccessorInfo> {
100//!         let am = AccessorInfo::default();
101//!         am.set_native_capability(
102//!             Capability {
103//!                 read: true,
104//!                 write: true,
105//!                 ..Default::default()
106//!         });
107//!
108//!         am.into()
109//!     }
110//! }
111//! ```
112//!
113//! Now that you have mastered [`Access`], let's go and implement our own backend!
114//!
115//! # Tutorial
116//!
117//! This tutorial implements a `duck` storage service that sends API
118//! requests to a super-powered duck. Gagaga!
119//!
120//! ## Scheme
121//!
122//! First of all, let's pick a good [`Scheme`] for our duck service. The
123//! scheme should be unique and easy to understand. Normally we should
124//! use its formal name.
125//!
126//! For example, we will use `s3` for AWS S3 Compatible Storage Service
127//! instead of `aws` or `awss3`. This is because there are many storage
128//! vendors that provide s3-like RESTful APIs, and our s3 service is
129//! implemented to support all of them, not just AWS S3.
130//!
131//! Obviously, we can use `duck` as scheme, let's add a new variant in [`Scheme`], and implement all required functions like `Scheme::from_str` and `Scheme::into_static`:
132//!
133//! ```ignore
134//! pub enum Scheme {
135//!     Duck,
136//! }
137//! ```
138//!
139//! ## Builder
140//!
141//! Then we can implement a builder for the duck service. The [`Builder`]
142//! will provide APIs for users to configure, and they will create an
143//! instance of a particular service.
144//!
145//! Let's create a `backend` mod under `services/duck` directory, and adding the following code.
146//!
147//! ```ignore
148//! use crate::raw::*;
149//! use crate::*;
150//!
151//! /// Duck Storage Service support. Gagaga!
152//! ///
153//! /// # Capabilities
154//! ///
155//! /// This service can be used to:
156//! ///
157//! /// - [x] read
158//! /// - [ ] write
159//! /// - [ ] list
160//! /// - [ ] presign
161//! ///
162//! /// # Configuration
163//! ///
164//! /// - `root`: Set the work dir for backend.
165//! ///
166//! /// ## Via Builder
167//! ///
168//! /// ```no_run
169//! /// use std::sync::Arc;
170//! ///
171//! /// use anyhow::Result;
172//! /// use opendal::services::Duck;
173//! /// use opendal::Operator;
174//! ///
175//! /// #[tokio::main]
176//! /// async fn main() -> Result<()> {
177//! ///     // Create Duck backend builder.
178//! ///     let mut builder = DuckBuilder::default();
179//! ///     // Set the root for duck, all operations will happen under this root.
180//! ///     //
181//! ///     // NOTE: the root must be absolute path.
182//! ///     builder.root("/path/to/dir");
183//! ///
184//! ///     let op: Operator = Operator::new(builder)?.finish();
185//! ///
186//! ///     Ok(())
187//! /// }
188//! /// ```
189//! #[derive(Default, Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
190//! #[serde(default)]
191//! #[non_exhaustive]
192//! pub struct DuckConfig {
193//!     pub root: Option<String>,
194//! }
195//!
196//! #[derive(Default, Clone)]
197//! pub struct DuckBuilder {
198//!     config: DuckConfig,
199//! }
200//! ```
201//!
202//! Now let's implement the required APIs for `DuckConfig`:
203//!
204//! ```ignore
205//! impl Configurator for DuckConfig {
206//!     type Builder = DuckBuilder;
207//!
208//!     fn into_builder(self) -> Self::Builder {
209//!         DuckBuilder { config: self }
210//!     }
211//! }
212//! ```
213//!
214//! Note that `DuckBuilder` is part of our public API, so it needs to be
215//! documented. And any changes you make will directly affect users, so
216//! please take it seriously. Otherwise, you will be hunted down by many
217//! angry ducks.
218//!
219//! Then, we can implement required APIs for `DuckBuilder`:
220//!
221//! ```ignore
222//! impl DuckBuilder {
223//!     /// Set root of this backend.
224//!     ///
225//!     /// All operations will happen under this root.
226//!     pub fn root(&mut self, root: &str) -> &mut Self {
227//!         self.config.root = if root.is_empty() {
228//!             None
229//!         } else {
230//!             Some(root.to_string())
231//!         };
232//!
233//!         self
234//!     }
235//! }
236//!
237//! impl Builder for DuckBuilder {
238//!     type Config = DuckConfig;
239//!
240//!     fn build(self) -> Result<impl Access>  {
241//!         debug!("backend build started: {:?}", &self);
242//!
243//!         let root = normalize_root(&self.config.root.clone().unwrap_or_default());
244//!         debug!("backend use root {}", &root);
245//!
246//!         Ok(DuckBackend { root })
247//!     }
248//! }
249//! ```
250//!
251//! `DuckBuilder` is ready now, let's try to play with real ducks!
252//!
253//! ## Backend
254//!
255//! I'm sure you can see it already: `DuckBuilder` will build a
256//! `DuckBackend` that implements [`Access`]. The backend is what we used
257//! to communicate with the super-powered ducks!
258//!
259//! Let's keep adding more code under `backend.rs`:
260//!
261//! ```ignore
262//! /// Duck storage service backend
263//! #[derive(Clone, Debug)]
264//! pub struct DuckBackend {
265//!     root: String,
266//! }
267//!
268//! impl Access for DuckBackend {
269//!     type Reader = DuckReader;
270//!     type Writer = ();
271//!     type Lister = ();
272//!     type Deleter = ();
273//!
274//!     fn info(&self) -> Arc<AccessorInfo> {
275//!         let am = AccessorInfo::default();
276//!         am.set_scheme("duck")
277//!             .set_root(&self.root)
278//!             .set_native_capability(
279//!                 Capability {
280//!                     read: true,
281//!                     ..Default::default()
282//!             });
283//!
284//!         am.into()
285//!     }
286//!
287//!     async fn read(&self, path: &str, args: OpRead) -> Result<(RpRead, Self::Reader)> {
288//!         gagaga!()
289//!     }
290//! }
291//! ```
292//!
293//! Congratulations, we have implemented an [`Access`] that can talk to
294//! Super Power Ducks!
295//!
296//! What!? There are no Super Power Ducks? So sad, but never mind, we have
297//! really powerful storage services [here](https://github.com/apache/opendal/issues/5). Welcome to pick one to implement. I promise you won't
298//! have to `gagaga!()` this time.
299//!
300//! [`Access`]: crate::raw::Access
301//! [`Operation`]: crate::raw::Operation
302//! [`Capability`]: crate::Capability
303//! [`AccessorInfo`]: crate::raw::AccessorInfo
304//! [`Scheme`]: crate::Scheme
305//! [`Builder`]: crate::Builder
306//! [`Configurator`]: crate::Configurator
```
