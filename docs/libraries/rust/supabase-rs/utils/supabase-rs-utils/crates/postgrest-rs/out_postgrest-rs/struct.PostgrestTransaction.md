# Struct PostgrestTransaction Copy item path

<a href="https://docs.rs/supabase-rust-postgrest/0.4.0/src/supabase_rust_postgrest/lib.rs.html#855-862" class="src">Source</a>

``` rust
pub struct PostgrestTransaction { /* private fields */ }
```

Expand description

トランザクションクライアント

## Implementations<a href="https://docs.rs/supabase-rust-postgrest/0.4.0/supabase_rust_postgrest/struct.PostgrestTransaction.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/supabase-rust-postgrest/0.4.0/supabase_rust_postgrest/struct.PostgrestTransaction.html#impl-PostgrestTransaction" class="anchor">§</a>

### impl <a href="https://docs.rs/supabase-rust-postgrest/0.4.0/supabase_rust_postgrest/struct.PostgrestTransaction.html" class="struct" title="struct supabase_rust_postgrest::PostgrestTransaction">PostgrestTransaction</a>

#### pub fn <a href="https://docs.rs/supabase-rust-postgrest/0.4.0/supabase_rust_postgrest/struct.PostgrestTransaction.html#method.from" class="fn">from</a>(&self, table: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://docs.rs/supabase-rust-postgrest/0.4.0/supabase_rust_postgrest/struct.PostgrestClient.html" class="struct" title="struct supabase_rust_postgrest::PostgrestClient">PostgrestClient</a>

トランザクション内で指定したテーブルに対するクライアントを取得

#### pub async fn <a href="https://docs.rs/supabase-rust-postgrest/0.4.0/supabase_rust_postgrest/struct.PostgrestTransaction.html#method.commit" class="fn">commit</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/supabase-rust-postgrest/0.4.0/supabase_rust_postgrest/enum.PostgrestError.html" class="enum" title="enum supabase_rust_postgrest::PostgrestError">PostgrestError</a>\>

トランザクションをコミット

#### pub async fn <a href="https://docs.rs/supabase-rust-postgrest/0.4.0/supabase_rust_postgrest/struct.PostgrestTransaction.html#method.rollback" class="fn">rollback</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/supabase-rust-postgrest/0.4.0/supabase_rust_postgrest/enum.PostgrestError.html" class="enum" title="enum supabase_rust_postgrest::PostgrestError">PostgrestError</a>\>

トランザクションをロールバック

#### pub async fn <a href="https://docs.rs/supabase-rust-postgrest/0.4.0/supabase_rust_postgrest/struct.PostgrestTransaction.html#method.savepoint" class="fn">savepoint</a>(&self, name: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/supabase-rust-postgrest/0.4.0/supabase_rust_postgrest/enum.PostgrestError.html" class="enum" title="enum supabase_rust_postgrest::PostgrestError">PostgrestError</a>\>

セーブポイントを作成

#### pub async fn <a href="https://docs.rs/supabase-rust-postgrest/0.4.0/supabase_rust_postgrest/struct.PostgrestTransaction.html#method.rollback_to_savepoint" class="fn">rollback_to_savepoint</a>( &self, name: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/supabase-rust-postgrest/0.4.0/supabase_rust_postgrest/enum.PostgrestError.html" class="enum" title="enum supabase_rust_postgrest::PostgrestError">PostgrestError</a>\>

セーブポイントにロールバック

## Trait Implementations<a href="https://docs.rs/supabase-rust-postgrest/0.4.0/supabase_rust_postgrest/struct.PostgrestTransaction.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/supabase-rust-postgrest/0.4.0/supabase_rust_postgrest/struct.PostgrestTransaction.html#impl-Drop-for-PostgrestTransaction" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html" class="trait" title="trait core::ops::drop::Drop">Drop</a> for <a href="https://docs.rs/supabase-rust-postgrest/0.4.0/supabase_rust_postgrest/struct.PostgrestTransaction.html" class="struct" title="struct supabase_rust_postgrest::PostgrestTransaction">PostgrestTransaction</a>

<a href="https://docs.rs/supabase-rust-postgrest/0.4.0/supabase_rust_postgrest/struct.PostgrestTransaction.html#method.drop" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html#tymethod.drop" class="fn">drop</a>(&mut self)

Executes the destructor for this type. [Read more](https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html#tymethod.drop)

## Auto Trait Implementations<a href="https://docs.rs/supabase-rust-postgrest/0.4.0/supabase_rust_postgrest/struct.PostgrestTransaction.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/supabase-rust-postgrest/0.4.0/supabase_rust_postgrest/struct.PostgrestTransaction.html#blanket-implementations" class="anchor">§</a>
